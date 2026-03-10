use std::io::Write;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::{io, mem, process};

use colored::Colorize;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};
use std::ffi::{CString, OsStr};
use sysinfo::{Signal, System};
use windows::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, TH32CS_SNAPTHREAD, THREADENTRY32, Thread32First, Thread32Next,
};
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::Win32::System::Memory::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, VirtualAllocEx};
use windows::Win32::System::Threading::{
    CreateRemoteThread, INFINITE, OpenProcess, OpenThread, PROCESS_ALL_ACCESS, SuspendThread,
    THREAD_SUSPEND_RESUME, WaitForSingleObject,
};
use windows::core::s;

use crate::components::configuration::configuration;
use crate::components::launcher::launcher;
use crate::components::settings::settings;
use crate::utils::proxy;

pub fn init_program() {
    loop {
        println!("[1] Launch a configuration");
        println!("[2] Create or Delete a configuration");
        println!("[3] Settings");
        println!("[4] Exit");

        print!("\n> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => launcher::init(),
            "2" => configuration::init(),
            "3" => settings::init(),
            "4" => {
                println!("Exiting...");
                let _ = kill_process("node.exe");
                let _ = kill_process("bun.exe");
                proxy::kill_proxy();
                process::exit(0);
            }
            _ => {
                println!("{}", "Invalid option".red());
                continue;
            }
        }
    }
}

pub fn clear_screen() {
    execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
}

pub fn launch_process(path: &str, args: Option<&str>) -> Child {
    let mut cmd = Command::new(path);

    if let Some(a) = args {
        cmd.args(a.split_whitespace());
    }

    cmd.stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap()
}

pub fn suspend_process(pid: u32) -> Result<(), String> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0)
            .map_err(|e| format!("Snapshot failed: {e}"))?;

        if snapshot == INVALID_HANDLE_VALUE {
            return Err("Invalid snapshot handle".into());
        }

        let mut entry = THREADENTRY32::default();
        entry.dwSize = mem::size_of::<THREADENTRY32>() as u32;

        if Thread32First(snapshot, &mut entry).is_err() {
            let _ = CloseHandle(snapshot);
            return Err("Thread32First failed".into());
        }

        loop {
            if entry.th32OwnerProcessID == pid {
                let thread = OpenThread(THREAD_SUSPEND_RESUME, false, entry.th32ThreadID);
                if let Ok(thread_handle) = thread {
                    SuspendThread(thread_handle);
                    let _ = CloseHandle(thread_handle);
                }
            }

            if Thread32Next(snapshot, &mut entry).is_err() {
                break;
            }
        }

        let _ = CloseHandle(snapshot);
        Ok(())
    }
}

pub fn inject_dll(pid: u32, path: &str) -> Result<(), String> {
    if !Path::new(path).exists() {
        return Err(format!("dll path does not exist: {}", path));
    }

    unsafe {
        let dll_path = CString::new(path).map_err(|e| e.to_string())?;
        let process = OpenProcess(PROCESS_ALL_ACCESS, false, pid)
            .map_err(|e| format!("OpenProcess failed: {e}"))?;
        let size = dll_path.as_bytes_with_nul().len();

        let remote_mem = VirtualAllocEx(
            process,
            None,
            size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        if remote_mem.is_null() {
            let _ = CloseHandle(process);
            return Err("VirtualAllocEx failed".into());
        }

        WriteProcessMemory(process, remote_mem, dll_path.as_ptr() as _, size, None).map_err(
            |e| {
                let _ = CloseHandle(process);
                format!("WriteProcessMemory failed: {e}")
            },
        )?;

        let kernel32 = GetModuleHandleA(s!("kernel32.dll"))
            .map_err(|e| format!("GetModuleHandleA failed: {e}"))?;

        let load_library =
            GetProcAddress(kernel32, s!("LoadLibraryA")).ok_or("GetProcAddress failed")?;

        let thread = CreateRemoteThread(
            process,
            None,
            0,
            Some(std::mem::transmute(load_library)),
            Some(remote_mem),
            0,
            None,
        )
        .map_err(|e| format!("CreateRemoteThread failed: {e}"))?;

        WaitForSingleObject(thread, INFINITE);

        let _ = CloseHandle(thread);
        let _ = CloseHandle(process);

        Ok(())
    }
}

pub fn kill_process(name: &str) -> Result<(), String> {
    let mut system = System::new_all();
    system.refresh_all();

    let mut found = false;
    for process in system.processes_by_name(OsStr::new(name)) {
        found = true;
        if process.kill_with(Signal::Kill).is_none() {
            process.kill();
        }
    }

    if !found {
        return Err(format!("Process '{}' not found", name));
    }

    Ok(())
}

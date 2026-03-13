use colored::Colorize;
use std::net::{TcpStream, ToSocketAddrs};
use std::os::windows::process::CommandExt;
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use std::{env, thread};

pub fn init_backend(backend: &str) -> Option<Child> {
    let current_dir = env::current_dir().unwrap();
    let mut backend_path = current_dir.clone();
    backend_path.push("backends");
    backend_path.push(backend);
    backend_path.push("run.bat");

    if !backend_path.exists() {
        println!("{} {}", "run.bat not found for backend:".red(), backend);
        return None;
    }

    let backend_dir = current_dir.join("backends").join(backend);

    let mut cmd = Command::new(backend_path);
    cmd.current_dir(backend_dir)
        .creation_flags(0x08000000)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null());

    match cmd.spawn() {
        Ok(child) => Some(child),
        Err(e) => {
            println!("{} {}", "Failed to start backend:".red(), e);
            None
        }
    }
}

pub fn wait_for_backend(addr: &str) {
    loop {
        let mut is_on = false;
        if let Ok(addrs) = addr.to_socket_addrs() {
            for addr in addrs {
                if addr.is_ipv4() && TcpStream::connect(addr).is_ok() {
                    is_on = true;
                    break;
                }
            }
        }

        if is_on {
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }
}

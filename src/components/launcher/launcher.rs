use std::{
    io::{self, Write},
    path::PathBuf,
    thread,
};

use colored::Colorize;

use crate::{
    components::launcher::{
        backend,
        game::{launch_fortnite, watch_output},
    },
    structs::{configuration::Configuration, settings::Settings},
    utils::{
        actions::{self, kill_process},
        configuration, proxy, settings,
    },
};

pub fn init() {
    actions::clear_screen();
    let configs = configuration::load().unwrap();
    let settings = settings::load_settings();

    loop {
        println!("Available configurations:");
        for config in configs.iter() {
            println!("  - {}: Path: {}", config.name, config.path);
        }

        print!("\nEnter the configuration name to launch (leave empty to return): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_str = input.trim();
        if input_str.is_empty() {
            actions::clear_screen();
            break;
        }

        let config = if let Some(pos) = configs
            .iter()
            .position(|c| c.name.eq_ignore_ascii_case(input_str))
        {
            &configs[pos]
        } else {
            println!("{}", "No configuration found with that name.".red());
            return;
        };

        loop {
            actions::clear_screen();
            println!("How would you like to launch that configuration:");
            println!("[1] Launch both client and gameserver");
            println!("[2] Launch client only");
            println!("[3] Launch gameserver only");
            println!("[0] Return");
            print!("\n> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input_str = input.trim();
            match input_str {
                "1" => launch_game(true, true, &settings, config),
                "2" => launch_game(true, false, &settings, config),
                "3" => launch_game(false, true, &settings, config),
                "0" => {
                    actions::clear_screen();
                    break;
                }
                _ => {
                    println!("{}", "Invalid option!.".red());
                }
            }
        }
    }
}

// proper spagetti code trust :pray:
fn launch_game(client: bool, gs: bool, settings: &Settings, config: &Configuration) {
    actions::clear_screen();

    let current_dir = std::env::current_dir().unwrap();
    let backend = settings.backend.as_deref().unwrap_or("").to_string();
    let backend_addr = match backend.as_str() {
        "LawinServer" => "localhost:3551",
        "Neonite" => "127.0.0.1:5595",
        "Nexa" => "127.0.0.1:5353",
        "Voltronite" => "127.0.0.1:8080",
        other => other,
    };

    println!("Waiting for backend to initialize...");
    let backend_thread = thread::spawn({
        let backend = backend.to_string();
        move || {
            let _ = kill_process("node.exe");
            let _ = kill_process("bun.exe");
            proxy::kill_proxy();
            if backend == "LawinServer"
                || backend == "Neonite"
                || backend == "Nexa"
                || backend == "Voltronite"
            {
                backend::init_backend(&backend);
            }

            if backend == "LawinServer" {
                proxy::init("127.0.0.1:3552".to_string()).unwrap();
            } else if backend == "Neonite" {
                proxy::init("127.0.0.1:5595".to_string()).unwrap();
            } else if backend == "Nexa" {
                proxy::init("127.0.0.1:5353".to_string()).unwrap();
            } else if backend == "Voltronite" {
                proxy::init("127.0.0.1:8080".to_string()).unwrap();
            } else {
                if !backend.ends_with("127.0.0.1:3551") {
                    proxy::init(backend.clone()).unwrap();
                }
            }
        }
    });

    backend_thread.join().unwrap();
    backend::wait_for_backend(backend_addr);
    println!("{}", "Backend initialized.".green());

    let redirect_dll: PathBuf = match settings.redirect.as_deref().unwrap_or("") {
        "Cobalt" => current_dir.join("dlls\\redirects\\cobalt.dll"),
        "Starfall" => current_dir.join("dlls\\redirects\\starfall.dll"),
        "Tellerium" => current_dir.join("dlls\\redirects\\tellerium.dll"),
        _ => {
            if let Some(ref redirect) = settings.redirect {
                PathBuf::from(redirect)
            } else {
                PathBuf::new()
            }
        }
    };

    let client_dll: PathBuf = match settings.clientdll.as_deref().unwrap_or("") {
        "Erbium" => current_dir.join("dlls\\client\\erbiumclient.dll"),
        "Default" => current_dir.join("dlls\\client\\console.dll"),
        _ => {
            if let Some(ref dll) = settings.clientdll {
                PathBuf::from(dll)
            } else {
                PathBuf::new()
            }
        }
    };

    let gs_dll: PathBuf = match settings.gameserver.as_deref().unwrap_or("") {
        "Erbium" => current_dir.join("dlls\\gameservers\\erbium.dll"),
        "Reboot" => current_dir.join("dlls\\gameservers\\reboot.dll"),
        _ => {
            if let Some(ref dll) = settings.gameserver {
                PathBuf::from(dll)
            } else {
                PathBuf::new()
            }
        }
    };

    let _ = kill_process("FortniteLauncher.exe");
    let _ = kill_process("FortniteClient-Win64-Shipping.exe");
    let _ = kill_process("FortniteClient-Win64-Shipping_EAC.exe");

    let fortnite_path = format!(
        "{}\\FortniteGame\\Binaries\\Win64\\FortniteClient-Win64-Shipping.exe",
        config.path
    );
    let fortnite_launcher_path = format!(
        "{}\\FortniteGame\\Binaries\\Win64\\FortniteLauncher.exe",
        config.path
    );
    let fortnite_eac_path = format!(
        "{}\\FortniteGame\\Binaries\\Win64\\FortniteClient-Win64-Shipping_EAC.exe",
        config.path
    );

    if client && gs {
        // client
        println!("Launching Fortnite....");
        let mut client_process = launch_fortnite(
            &fortnite_launcher_path,
            &fortnite_path,
            &fortnite_eac_path,
            config,
            redirect_dll.to_str().unwrap(),
            false,
        );

        // gs
        println!("Launching gameserver....");
        let mut gs_process = launch_fortnite(
            &fortnite_launcher_path,
            &fortnite_path,
            &fortnite_eac_path,
            config,
            redirect_dll.to_str().unwrap(),
            true,
        );

        let client_pid = client_process.id();
        let client_stdout = client_process.stdout.take().unwrap();
        watch_output(
            client_stdout,
            client_pid,
            client_dll.to_str().unwrap().to_string(),
            false,
        );

        let gs_pid = gs_process.id();
        let gs_stdout = gs_process.stdout.take().unwrap();
        watch_output(
            gs_stdout,
            gs_pid,
            gs_dll.to_str().unwrap().to_string(),
            true,
        );

        match client_process.wait() {
            Ok(_) => {
                println!("Fortnite closed.");
                let _ = kill_process("FortniteLauncher.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping_EAC.exe");
            }
            Err(_) => {
                println!("Something went wrong, killing Fortnite...");
                let _ = kill_process("FortniteLauncher.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping_EAC.exe");
            }
        }

        match gs_process.wait() {
            Ok(_) => {
                println!("Fortnite closed.");
                let _ = kill_process("FortniteLauncher.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping_EAC.exe");
            }
            Err(_) => {
                println!("Something went wrong, killing Fortnite...");
                let _ = kill_process("FortniteLauncher.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping_EAC.exe");
            }
        }
    } else if client && !gs {
        println!("Launching Fortnite....");
        let mut client_process = launch_fortnite(
            &fortnite_launcher_path,
            &fortnite_path,
            &fortnite_eac_path,
            config,
            redirect_dll.to_str().unwrap(),
            false,
        );

        let client_pid = client_process.id();
        let client_stdout = client_process.stdout.take().unwrap();
        watch_output(
            client_stdout,
            client_pid,
            client_dll.to_str().unwrap().to_string(),
            false,
        );

        match client_process.wait() {
            Ok(_) => {
                println!("Fortnite closed.");
                let _ = kill_process("FortniteLauncher.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping_EAC.exe");
            }
            Err(_) => {
                println!("Something went wrong, killing Fortnite...");
                let _ = kill_process("FortniteLauncher.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping_EAC.exe");
            }
        }
    } else {
        println!("Launching gameserver....");
        let mut gs_process = launch_fortnite(
            &fortnite_launcher_path,
            &fortnite_path,
            &fortnite_eac_path,
            config,
            redirect_dll.to_str().unwrap(),
            true,
        );

        let gs_pid = gs_process.id();
        let gs_stdout = gs_process.stdout.take().unwrap();
        watch_output(
            gs_stdout,
            gs_pid,
            gs_dll.to_str().unwrap().to_string(),
            true,
        );

        match gs_process.wait() {
            Ok(_) => {
                println!("Fortnite closed.");
                let _ = kill_process("FortniteLauncher.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping_EAC.exe");
            }
            Err(_) => {
                println!("Something went wrong, killing Fortnite...");
                let _ = kill_process("FortniteLauncher.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping.exe");
                let _ = kill_process("FortniteClient-Win64-Shipping_EAC.exe");
            }
        }
    }
}

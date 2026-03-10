use std::{
    io::{BufRead, BufReader},
    process::{Child, ChildStdout},
    thread,
};

use colored::Colorize;

use crate::{
    components::launcher::{CANNOT_CONNECT_ERRORS, LOGGED_IN_LINES},
    structs::configuration::Configuration,
    utils::actions,
};

pub fn launch_fortnite(
    fortnite_launcher_path: &String,
    fortnite_path: &String,
    fortnite_eac_path: &String,
    config: &Configuration,
    redirect_dll: &str,
    is_gs: bool,
) -> Child {
    let fortnite_launcher_process = actions::launch_process(fortnite_launcher_path.as_str(), None);
    match actions::suspend_process(fortnite_launcher_process.id()) {
        Ok(()) => {
            println!("{}", "FortniteLauncher.exe suspended successfully.".green());
        }
        Err(e) => {
            eprintln!("{}", format!("Failed to suspend process: {:?}", e).red());
        }
    }

    let fortnite_eac_process = actions::launch_process(fortnite_eac_path.as_str(), None);
    match actions::suspend_process(fortnite_eac_process.id()) {
        Ok(()) => {
            println!(
                "{}",
                "FortniteClient-Win64-Shipping_EAC.exe suspended successfully.".green()
            );
        }
        Err(e) => {
            eprintln!("{}", format!("Failed to suspend process: {:?}", e).red());
        }
    }

    let fortnite_process = actions::launch_process(
        fortnite_path.as_str(),
        Some(&args(config.email.clone(), config.password.clone(), is_gs)),
    );

    // TODO: figure out how to find ch1 builds so i can use this
    /*
    match actions::inject_dll(
        fortnite_process.id(),
        current_dir
            .join("dlls")
            .join("memory.dll")
            .to_str()
            .unwrap(),
    ) {
        Ok(()) => println!("{}", "Injected Memcury successfully.".green()),
        Err(e) => eprintln!("{}", format!("Failed to inject Memcury: {:?}", e).red()),
    }
    */

    match actions::inject_dll(fortnite_process.id(), redirect_dll) {
        Ok(()) => println!("{}", "Injected Redirect dll successfully.".green()),
        Err(e) => eprintln!(
            "{}",
            format!("Failed to inject Redirect dll: {:?}", e).red()
        ),
    }

    fortnite_process
}

pub fn watch_output(stdout: ChildStdout, inject_pid: u32, dll: String, is_gs: bool) {
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().flatten() {
            if is_gs && CANNOT_CONNECT_ERRORS.iter().any(|e| line.contains(e)) {
                println!(
                    "{}",
                    "Authentification error detected, killing process...".red()
                );
                let _ = actions::kill_process("FortniteClient-Win64-Shipping.exe");
                break;
            }

            if LOGGED_IN_LINES.iter().any(|e| line.contains(e)) {
                println!(
                    "{}",
                    format!(
                        "{} dll injected...",
                        if is_gs { "Gameserver" } else { "Client" }
                    )
                    .green()
                );
                let _ = actions::inject_dll(inject_pid, &dll);
                break;
            }
        }
    });
}

fn args(email: String, password: String, is_gs: bool) -> String {
    if is_gs {
        format!(
            "-epicapp=Fortnite -epicenv=Prod -epicportal -AUTH_TYPE=epic -AUTH_LOGIN={} -AUTH_PASSWORD={} -epiclocale=en-us -fltoken=7a848a93a74ba68876c36C1c -fromfl=none -noeac -nobe -skippatchcheck -nullrhi -nosound -nosplash",
            email, password
        )
    } else {
        format!(
            "-epicapp=Fortnite -epicenv=Prod -epicportal -AUTH_TYPE=epic -AUTH_LOGIN={} -AUTH_PASSWORD={} -epiclocale=en-us -fltoken=7a848a93a74ba68876c36C1c -fromfl=none -noeac -nobe -skippatchcheck",
            email, password
        )
    }
}

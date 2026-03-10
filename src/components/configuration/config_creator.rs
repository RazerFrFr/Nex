use crate::{structs::configuration::Configuration, utils::actions, utils::configuration};
use colored::Colorize;
use regex::Regex;
use std::{
    io::{self, Write},
    path::Path,
};

enum State {
    Name,
    Email,
    Password,
    GsEmail,
    GsPassword,
    Path,
}

pub fn init() {
    'config_loop: loop {
        actions::clear_screen();

        let mut state = State::Name;
        let mut config = Configuration::new();
        let old_configs = configuration::load().unwrap();

        loop {
            match state {
                State::Name => {
                    println!("Set the name of the new configuration");
                    loop {
                        print!("Name: ");
                        io::stdout().flush().unwrap();

                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        let input_str = input.trim();

                        if input_str.is_empty() {
                            println!("{}", "Name cannot be empty!".red());
                            continue;
                        }

                        if old_configs
                            .iter()
                            .any(|c| c.name.eq_ignore_ascii_case(input_str))
                        {
                            println!("{}", "Error: Name is already in use".red());
                            continue;
                        }

                        config.name = input_str.to_string();
                        state = State::Email;
                        break;
                    }
                }
                State::Email => {
                    actions::clear_screen();
                    println!("Set the email to connect to the backend for this configuration");
                    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
                    loop {
                        print!("Email: ");
                        io::stdout().flush().unwrap();

                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        let input_str = input.trim();

                        if input_str.is_empty() {
                            println!("{}", "Email cannot be empty!".red());
                            continue;
                        }

                        if !email_regex.is_match(input_str) {
                            println!("{}", "Error: Invalid email format!".red());
                            continue;
                        }

                        config.email = input_str.to_string();
                        state = State::Password;
                        break;
                    }
                }
                State::Password => {
                    actions::clear_screen();
                    println!("Set the password for the email of this configuration");
                    print!("Password: ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input_str = input.trim();

                    if !input_str.is_empty() {
                        config.password = input_str.to_string();
                        state = State::GsEmail;
                    }
                }
                State::GsEmail => {
                    actions::clear_screen();
                    println!(
                        "Set the gameserver email to connect to the backend for this configuration (can be empty)"
                    );
                    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
                    loop {
                        print!("Email: ");
                        io::stdout().flush().unwrap();

                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        let input_str = input.trim();

                        if input_str.is_empty() {
                            config.gs_email = String::new();
                            state = State::GsPassword;
                            break;
                        }

                        if !email_regex.is_match(input_str) {
                            println!("{}", "Error: Invalid email format!".red());
                            continue;
                        }

                        config.gs_email = input_str.to_string();
                        state = State::GsPassword;
                        break;
                    }
                }
                State::GsPassword => {
                    actions::clear_screen();
                    println!(
                        "Set the gameserver password for the gameserver email of this configuration (can be empty)"
                    );
                    print!("Password: ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input_str = input.trim();

                    if !input_str.is_empty() {
                        config.gs_password = input_str.to_string();
                    } else {
                        config.gs_password = String::new();
                    }

                    state = State::Path;
                }
                State::Path => {
                    actions::clear_screen();
                    println!("Set the path of the build you wanna use for this configuration");
                    loop {
                        print!("Path: ");
                        io::stdout().flush().unwrap();

                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        let input_str = input.trim();

                        if input_str.is_empty() {
                            println!("{}", "Path cannot be empty!".red());
                            continue;
                        }

                        let path = Path::new(input_str);
                        if !path.exists() || !path.is_dir() {
                            println!("{}", "Error: invalid folder!".red());
                            continue;
                        }

                        let engine_path = path.join("Engine");
                        let game_path = path.join("FortniteGame");
                        if !engine_path.exists() || !engine_path.is_dir() {
                            println!("{}", "Error: invalid folder!".red());
                            continue;
                        }

                        if !game_path.exists() || !game_path.is_dir() {
                            println!("{}", "Error: invalid folder!".red());
                            continue;
                        }

                        config.path = input_str.to_string();
                        break;
                    }

                    break;
                }
            }
        }

        loop {
            actions::clear_screen();
            println!(
                "Name: {}\nEmail: {}\nPassword: {}\nPath: {}",
                config.name, config.email, config.password, config.path
            );

            println!("Is this configuration correct? (y/n, default: y)");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input_str = input.trim().to_lowercase();

            match input_str.as_str() {
                "" | "y" => {
                    configuration::save(config).unwrap();
                    actions::clear_screen();
                    println!("{}", "Successfully saved the configuration".green());
                    break 'config_loop;
                }
                "n" => continue 'config_loop,
                _ => {
                    println!("{}", "Invalid input! Please type y or n.".red());
                    continue;
                }
            }
        }
    }
}

use std::io::{self, Write};

use colored::Colorize;
use url::Url;

use crate::{utils::actions, utils::settings};

pub fn init() {
    let settings = settings::load_settings();
    actions::clear_screen();

    loop {
        println!(
            "Current selected backend: {}",
            settings.backend.as_deref().unwrap_or("None")
        );
        println!("\nSelect a new backend:");
        println!("[1] LawinServerV1");
        println!("[2] Neonite");
        println!("[3] Nexa");
        println!("[4] Voltronite");
        println!("[5] Custom");
        println!("[0] Return");
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {
                let _ = settings::save_setting("LawinServer".to_string(), "backend");
                actions::clear_screen();
                println!("{}", "Backend successfully saved".green());
                break;
            }
            "2" => {
                let _ = settings::save_setting("Neonite".to_string(), "backend");
                actions::clear_screen();
                println!("{}", "Backend successfully saved".green());
                break;
            }
            "3" => {
                let _ = settings::save_setting("Nexa".to_string(), "backend");
                actions::clear_screen();
                println!("{}", "Backend successfully saved".green());
                break;
            }
            "4" => {
                let _ = settings::save_setting("Voltronite".to_string(), "backend");
                actions::clear_screen();
                println!("{}", "Backend successfully saved".green());
                break;
            }
            "5" => {
                actions::clear_screen();
                println!("Enter the backend url (eg: http://127.0.0.1:8080)");
                print!("\nBackend Url: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                match Url::parse(input) {
                    Ok(url) if url.scheme() == "http" || url.scheme() == "https" => {
                        let _ = settings::save_setting(input.to_string(), "backend");
                        println!("{}", "Backend successfully saved".green());
                        break;
                    }
                    _ => {
                        println!("{}", "Invalid URL. Example: http://127.0.0.1:8080".red());
                        continue;
                    }
                }
            }
            "0" => {
                actions::clear_screen();
                break;
            }
            _ => {
                println!("{}", "Invalid option".red());
                continue;
            }
        }
    }
}

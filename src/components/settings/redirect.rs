use std::{
    io::{self, Write},
    path::Path,
};

use colored::Colorize;

use crate::{utils::actions, utils::settings};

pub fn init() {
    let settings = settings::load_settings();
    actions::clear_screen();

    loop {
        println!(
            "Current selected redirect: {}",
            settings.redirect.as_deref().unwrap_or("None")
        );
        println!("\nSelect a new redirect:");
        println!("[1] Cobalt (legacy)");
        println!("[2] Starfall");
        println!("[3] Tellerium");
        println!("[4] Custom");
        println!("[0] Return");
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                let _ = settings::save_setting("Cobalt".to_string(), "redirect");
                actions::clear_screen();
                println!("{}", "Redirect successfully saved".green());
                break;
            }
            "2" => {
                let _ = settings::save_setting("Starfall".to_string(), "redirect");
                actions::clear_screen();
                println!("{}", "Redirect successfully saved".green());
                break;
            }
            "3" => {
                let _ = settings::save_setting("Tellerium".to_string(), "redirect");
                actions::clear_screen();
                println!("{}", "Redirect successfully saved".green());
                break;
            }
            "4" => {
                actions::clear_screen();
                println!("Set the path to the redirect dll");
                print!("\nPath: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                if input.is_empty() {
                    println!("{}", "Path cannot be empty!".red());
                    continue;
                }

                let path = Path::new(input);
                if !path.exists()
                    || !path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext.eq_ignore_ascii_case("dll"))
                        .unwrap_or(false)
                {
                    println!("{}", "Error: invalid file!".red());
                    continue;
                }

                let _ = settings::save_setting(input.to_string(), "redirect");
                println!("{}", "Redirect successfully saved".green());
                break;
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

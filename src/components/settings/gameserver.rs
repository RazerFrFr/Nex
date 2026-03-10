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
            "Current selected gameserver: {}",
            settings.gameserver.as_deref().unwrap_or("None")
        );
        println!("\nSelect a new gameserver:");
        println!("[1] Erbium");
        println!("[2] Reboot");
        println!("[3] Custom");
        println!("[0] Return");
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                let _ = settings::save_setting("Erbium".to_string(), "gameserver");
                actions::clear_screen();
                println!("{}", "Gameserver successfully saved".green());
                break;
            }
            "2" => {
                let _ = settings::save_setting("Reboot".to_string(), "gameserver");
                actions::clear_screen();
                println!("{}", "Gameserver successfully saved".green());
                break;
            }
            "3" => {
                actions::clear_screen();
                println!("Set the path to the gameserver dll");
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

                let _ = settings::save_setting(input.to_string(), "gameserver");
                println!("{}", "Gameserver successfully saved".green());
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

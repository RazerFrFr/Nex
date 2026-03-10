use colored::Colorize;

use crate::{utils::actions, utils::configuration};
use std::io::{self, Write};

pub fn init() {
    let mut configs = configuration::load().unwrap();

    loop {
        actions::clear_screen();
        println!("Available configurations:");
        for config in configs.iter() {
            println!("  - {}: Path: {}", config.name, config.path);
        }

        print!("\nEnter the configuration name to delete (leave empty to cancel): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_str = input.trim();
        if input_str.is_empty() {
            break;
        }

        if let Some(pos) = configs
            .iter()
            .position(|c| c.name.eq_ignore_ascii_case(input_str))
        {
            configs.remove(pos);
            configuration::save_all(&configs).unwrap();
            actions::clear_screen();
            println!("{}", "Successfully deleted the configuration".green());
            break;
        } else {
            println!("{}", "No configuration found with that name.".red());
            // loop continues for retry
        }
    }
}

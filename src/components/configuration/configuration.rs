use colored::Colorize;
use std::io::{self, Write};

use crate::{
    components::configuration::{config_creator, config_deleter},
    utils::actions,
};

pub fn init() {
    actions::clear_screen();

    loop {
        println!("[1] Create a configuration");
        println!("[2] Delete a configuration");
        println!("[0] Return");
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => config_creator::init(),
            "2" => config_deleter::init(),
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

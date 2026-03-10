use crate::{
    components::settings::{backend, client, gameserver, redirect},
    utils::actions,
};
use colored::Colorize;
use std::io::{self, Write};

pub fn init() {
    actions::clear_screen();

    loop {
        println!("[1] Choose current backend");
        println!("[2] Choose current gameserver");
        println!("[3] Choose current redirect");
        println!("[4] Choose current client dll");
        println!("[0] Return");
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => backend::init(),
            "2" => gameserver::init(),
            "3" => redirect::init(),
            "4" => client::init(),
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

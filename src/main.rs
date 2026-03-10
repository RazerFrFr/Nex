use crossterm::{execute, terminal::SetTitle};
use figlet_rs::FIGfont;
use std::io::stdout;

use crate::utils::{actions, settings};
mod components;
mod structs;
mod utils;

fn main() {
    // first launch
    let settings = settings::load_settings();
    if settings.backend.is_none()
        && settings.gameserver.is_none()
        && settings.redirect.is_none()
        && settings.clientdll.is_none()
    {
        settings::save_setting("LawinServer".to_string(), "backend").ok();
        settings::save_setting("Erbium".to_string(), "gameserver").ok();
        settings::save_setting("Tellerium".to_string(), "redirect").ok();
        settings::save_setting("Erbium".to_string(), "clientdll").ok();
    }

    let _ = execute!(stdout(), SetTitle("Nex - Made by Razer"));

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Nex");
    println!("{}", figure.unwrap());
    println!("Made with love by Razer. Github: https://github.com/RazerFrFr\n\n");

    loop {
        actions::init_program();
    }
}

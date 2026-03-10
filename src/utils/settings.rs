use std::{fs, io::Result, path::PathBuf};

use crate::structs::settings::Settings;

pub fn load_settings() -> Settings {
    let mut folder = PathBuf::from(std::env::var_os("APPDATA").unwrap());
    folder.push("Nex");
    folder.push("settings.json");

    if !folder.exists() {
        return Settings::default();
    }

    let content = match fs::read_to_string(folder) {
        Ok(c) => c,
        Err(_) => return Settings::default(),
    };

    let mut settings: Settings = match serde_json::from_str(&content) {
        Ok(s) => s,
        Err(_) => return Settings::default(),
    };

    settings.redirect = normalize(settings.redirect);
    settings.backend = normalize(settings.backend);
    settings.gameserver = normalize(settings.gameserver);
    settings.clientdll = normalize(settings.clientdll);

    settings
}

pub fn save_setting(value: String, setting_type: &str) -> Result<()> {
    let mut path = PathBuf::from(std::env::var_os("APPDATA").unwrap());

    path.push("Nex");
    std::fs::create_dir_all(&path).ok();

    path.push("settings.json");
    let mut settings = load_settings();

    let val = if value.trim().is_empty() {
        None
    } else {
        Some(value)
    };

    match setting_type {
        "backend" => settings.backend = val,
        "redirect" => settings.redirect = val,
        "gameserver" => settings.gameserver = val,
        "clientdll" => settings.clientdll = val,
        _ => (),
    }

    let json = serde_json::to_string_pretty(&settings)?;
    fs::write(path, json)?;
    Ok(())
}

fn normalize(value: Option<String>) -> Option<String> {
    match value {
        Some(v) if !v.trim().is_empty() => Some(v),
        _ => None,
    }
}

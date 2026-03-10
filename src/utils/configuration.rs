use serde_json::{Value, json};
use std::{
    fs::{self, OpenOptions},
    io::{Read, Result, Write},
    path::PathBuf,
};

use crate::structs::configuration::Configuration;

pub fn save(config: Configuration) -> Result<()> {
    let mut folder = PathBuf::from(std::env::var_os("APPDATA").unwrap());
    folder.push("Nex");
    if !folder.exists() {
        fs::create_dir_all(&folder)?;
    }

    let mut config_file_path = folder.clone();
    config_file_path.push("configs.json");

    let mut root: Value = if config_file_path.exists() {
        let mut file = OpenOptions::new().read(true).open(&config_file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).unwrap_or_else(|_| json!({"configurations": []}))
    } else {
        serde_json::json!({"configurations": []})
    };

    if let Some(array) = root
        .get_mut("configurations")
        .and_then(|v| v.as_array_mut())
    {
        array.push(serde_json::to_value(&config)?);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&config_file_path)?;

    file.write_all(serde_json::to_string_pretty(&root)?.as_bytes())?;
    Ok(())
}

pub fn load() -> Result<Vec<Configuration>> {
    let mut folder = PathBuf::from(std::env::var_os("APPDATA").unwrap());
    folder.push("Nex");

    let mut file_path = folder.clone();
    file_path.push("configs.json");
    if !file_path.exists() {
        return Ok(Vec::new());
    }

    let mut file = OpenOptions::new().read(true).open(&file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let root: Value = serde_json::from_str(&contents)
        .unwrap_or_else(|_| serde_json::json!({"configurations": []}));

    let configs: Vec<Configuration> = root
        .get("configurations")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|val| serde_json::from_value(val.clone()).ok())
                .collect()
        })
        .unwrap_or_default();

    Ok(configs)
}

pub fn save_all(configs: &[Configuration]) -> Result<()> {
    let mut folder = PathBuf::from(std::env::var_os("APPDATA").unwrap());
    folder.push("Nex");
    if !folder.exists() {
        fs::create_dir_all(&folder)?;
    }

    let mut config_file_path = folder.clone();
    config_file_path.push("configs.json");
    let root = json!({
        "configurations": configs
    });

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&config_file_path)?;

    file.write_all(serde_json::to_string_pretty(&root)?.as_bytes())?;
    Ok(())
}

use std::{
    fmt::write,
    fs::{self},
    path::PathBuf,
};

use directories::ProjectDirs;

use crate::commands::config_structure::{Config, TimesOfDay};

fn config_dir() -> PathBuf {
    let path = ProjectDirs::from("com", "vaarischitkara", "shift")
        .expect("Could not determine config directory");

    path.config_dir().to_path_buf()
}

pub fn get_config_path() -> PathBuf {
    let dir = config_dir();
    fs::create_dir_all(&dir);

    dir.join("config_shift.toml")
}

pub fn create_toml() -> std::io::Result<PathBuf> {
    let config_path = get_config_path();

    if !config_path.exists() {
        let toml_content = toml::to_string_pretty(&Config::default()).unwrap();

        fs::write(&config_path, toml_content)?;
    }

    Ok(config_path)
}

pub fn edit_toml(field: TimesOfDay, path: Option<PathBuf>) -> std::io::Result<()> {
    let config_file = create_toml()?;

    let content = fs::read_to_string(&config_file)?;

    let mut config: Config = toml::from_str(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    if let Some(path) = path {
        let path_string = path.to_string_lossy().to_string();
        match field {
            TimesOfDay::Morning => config.morning_wallpaper = path_string,
            TimesOfDay::Afternoon => config.afternoon_wallpaper = path_string,
            TimesOfDay::Evening => config.evening_wallpaper = path_string,
        }

        let toml_content = toml::to_string_pretty(&config)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        fs::write(&config_file, toml_content)?;
    }

    Ok(())
}

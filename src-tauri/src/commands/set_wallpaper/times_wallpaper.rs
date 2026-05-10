use std::fs;

use crate::commands::{
    config_structure::Config, make_path::make_toml::get_config_path, utilities::get_wallpaper::*,
};

pub fn load_config() -> Config {
    let content = fs::read_to_string(get_config_path()).expect("Failed to read file");

    toml::from_str(&content).expect("Failed to parse TOML")
}

pub fn set_morning_wallpaper() {
    let path = get_wallpaper_morning();

    wallpaper::set_from_path(&path).expect("Error setting morning_wallpaper");
}

pub fn set_afternoon_wallpaper() {
    let path = get_wallpaper_afternoon();

    wallpaper::set_from_path(&path).expect("Error setting afternoon_wallpaper");
}

pub fn set_evening_wallpaper() {
    let path = get_wallpaper_evening();

    wallpaper::set_from_path(&path).expect("Error setting evening_wallpaper");
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::open_file_explorer::open_file_explorer::open_file_explorer;

use crate::commands::{
    make_path::make_toml::create_toml, set_wallpaper::set_wallpaper::set_wallpaper,
};

use crate::commands::utilities::get_wallpaper::*;

fn main() {
    create_toml().expect("Errrrrorrr");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_file_explorer,
            get_wallpaper_morning,
            get_wallpaper_afternoon,
            get_wallpaper_evening
        ])
        .setup(|_| {
            set_wallpaper();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

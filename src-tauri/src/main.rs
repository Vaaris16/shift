// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::open_file_explorer::open_file_explorer::open_file_explorer;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;

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
        .setup(|app| {
            TrayIconBuilder::new().build(app)?;
            set_wallpaper();
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                window.hide().unwrap();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use crate::commands::set_wallpaper::times_wallpaper::load_config;

#[tauri::command]
pub fn get_wallpaper_morning() -> String {
    let config = load_config();

    config.morning_wallpaper
}

#[tauri::command]
pub fn get_wallpaper_afternoon() -> String {
    let config = load_config();

    config.afternoon_wallpaper
}

#[tauri::command]
pub fn get_wallpaper_evening() -> String {
    let config = load_config();

    config.evening_wallpaper
}

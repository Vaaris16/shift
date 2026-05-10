use crate::commands::{config_structure::TimesOfDay, make_path::make_toml::edit_toml};
use rfd::FileDialog;

#[tauri::command]
pub async fn open_file_explorer(field: TimesOfDay) {
    let path = FileDialog::new()
        .add_filter("Images", &["png", "jpg", "jpeg"])
        .set_directory("/")
        .pick_file();

    println!("path: {:?}", path);
    edit_toml(field, path).expect("edit_toml failed at open_file_explorer");
    println!("toml written");
}

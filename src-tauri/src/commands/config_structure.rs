use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    pub morning_wallpaper: String,
    pub afternoon_wallpaper: String,
    pub evening_wallpaper: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimesOfDay {
    Morning,
    Afternoon,
    Evening,
}

impl Config {
    pub fn default() -> Self {
        Self {
            morning_wallpaper: String::new(),
            afternoon_wallpaper: String::new(),
            evening_wallpaper: String::new(),
        }
    }
}

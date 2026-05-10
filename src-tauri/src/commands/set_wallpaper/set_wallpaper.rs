use std::time::Duration;

use chrono::{Local, NaiveTime};

use crate::commands::set_wallpaper::times_wallpaper::{
    set_afternoon_wallpaper, set_evening_wallpaper, set_morning_wallpaper,
};

pub fn set_wallpaper() {
    tauri::async_runtime::spawn(async move {
        loop {
            let now = Local::now().time();

            let morning_start = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
            let morning_end = NaiveTime::from_hms_opt(12, 0, 0).unwrap();

            let afternoon_start = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
            let afternoon_end = NaiveTime::from_hms_opt(18, 0, 0).unwrap();

            let evening_start = NaiveTime::from_hms_opt(18, 0, 0).unwrap();
            let evening_end = NaiveTime::from_hms_opt(23, 59, 59).unwrap();

            if morning_start <= now && now <= morning_end {
                set_morning_wallpaper();
            } else if afternoon_start <= now && now <= afternoon_end {
                set_afternoon_wallpaper();
            } else if evening_start <= now && now <= evening_end {
                set_evening_wallpaper();
            } else {
                println!("Error in set_wallpaper");
            }
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
}

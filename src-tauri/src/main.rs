#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod db;
mod player;
mod tuner;

use anyhow::Result;
use commands::*;
use db::Db;
use player::Player;
use std::sync::Mutex;
use tauri::Manager;
use tuner::Tuner;

pub struct RadioState {
    tuner: Mutex<Tuner>,
    player: Mutex<Player>,
    db: Mutex<Db>,
}

fn main() -> Result<()> {
    let state = RadioState {
        tuner: Mutex::new(Tuner::new()?),
        player: Mutex::new(Player::new()),
        db: Mutex::new(Db::new()),
    };

    tauri::Builder::default()
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                std::thread::sleep(std::time::Duration::from_secs(5));

                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });

            Ok(())
        })
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            search_stations,
            play_station,
            play,
            pause,
            stream_events,
            bookmark_station,
            remove_bookmark_station,
            bookmark_stations_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

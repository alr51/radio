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
use tuner::Tuner;
use log::debug;

pub struct RadioState {
    tuner: Mutex<Tuner>,
    player: Mutex<Player>,
    db: Mutex<Db>,
}

fn main() -> Result<()> {
    env_logger::init();

    let state = RadioState {
        tuner: Mutex::new(Tuner::new()?),
        player: Mutex::new(Player::new()),
        db: Mutex::new(Db::new()),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            search_stations,
            play_station,
            play,
            pause,
            stream_events,
            bookmark_station,
            remove_bookmark_station,
            bookmark_stations_list,
            set_volume,
            mute
        ])
        .on_window_event(move |event| match event.event() {
            tauri::WindowEvent::CloseRequested { .. } => {
                debug!("close requested");
                std::process::exit(0);
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod db;
mod player;
mod tuner;
mod musicbrainz;
mod fanarttv;
mod wikipedia;

use anyhow::Result;
use commands::*;
use db::Db;
use fanarttv::FanArtTv;
use musicbrainz::MusicBrainz;
use player::Player;
use std::sync::Mutex;
use tuner::Tuner;
use log::debug;
use dotenv::dotenv;
use wikipedia::Wikipedia;


pub const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"), " (", env!("CARGO_PKG_REPOSITORY"), ")");


pub const MUSICBRAINZ_API_ENDPOINT: &str = "https://musicbrainz.org/ws/2/";

pub struct RadioState {
    tuner: Mutex<Tuner>,
    player: Mutex<Player>,
    db: Mutex<Db>,
    mb: Mutex<MusicBrainz>,
    fatv: Mutex<FanArtTv>,
    wiki: Mutex<Wikipedia>,
}

fn main() -> Result<()> {
    
    dotenv().ok();

    env_logger::init();

    let state = RadioState {
        tuner: Mutex::new(Tuner::new()?),
        player: Mutex::new(Player::new()),
        db: Mutex::new(Db::new()),
        mb: Mutex::new(MusicBrainz::new()),
        fatv: Mutex::new(FanArtTv::new()),
        wiki: Mutex::new(Wikipedia::new())
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
            mute,
            artist_info
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

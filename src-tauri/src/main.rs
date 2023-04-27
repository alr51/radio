#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod db;
mod fanarttv;
mod musicbrainz;
mod player;
mod tuner;
mod wikipedia;

use anyhow::Result;
use commands::*;
use db::Db;
use dotenv::dotenv;
use fanarttv::FanArtTv;
use log::debug;
use musicbrainz::MusicBrainz;
use player::Player;
use std::sync::Mutex;
use tuner::Tuner;
use wikipedia::Wikipedia;

pub const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_REPOSITORY"),
    ")"
);

pub struct RadioState {
    tuner: tokio::sync::Mutex<Tuner>,
    player: Mutex<Player>,
    db: Mutex<Db>,
    mb: tokio::sync::Mutex<MusicBrainz>,
    fatv: tokio::sync::Mutex<FanArtTv>,
    wiki: tokio::sync::Mutex<Wikipedia>,
}
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    env_logger::init();

    let state = RadioState {
        tuner: tokio::sync::Mutex::new(Tuner::new().await?),
        player: Mutex::new(Player::new()),
        db: Mutex::new(Db::new()),
        mb: tokio::sync::Mutex::new(MusicBrainz::new()),
        fatv: tokio::sync::Mutex::new(FanArtTv::new()),
        wiki: tokio::sync::Mutex::new(Wikipedia::new()),
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
            artist_info,
            artist_releases,
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

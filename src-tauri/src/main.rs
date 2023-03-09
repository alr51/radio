#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use anyhow::Result;
use player::Player;
use tauri::{Manager, Window};
use tuner::{Station, StationsSearchQuery, Tuner};

use gstreamer::prelude::ObjectExt;

mod player;
mod tuner;

struct RadioState {
    tuner: Mutex<Tuner>,
    player: Mutex<Player>,
}

#[tauri::command]
fn search_stations(state: tauri::State<RadioState>, stations_query: StationsSearchQuery) -> Vec<Station> {
    // println!("search_stations");
    if let Ok(stations) = state
        .tuner
        .lock()
        .expect("no tuner found")
        .search(stations_query)
    {
        return stations.to_vec();
    }
    vec![]
}

#[tauri::command]
fn pause(state: tauri::State<RadioState>) {
    let _ = state.player.lock().expect("no player found").pause();
}

#[tauri::command]
fn play(state: tauri::State<RadioState>) {
    let _ = state.player.lock().expect("no player found").play();
}

#[tauri::command]
fn stream_events(state: tauri::State<RadioState>, window: Window) {
    state
        .player
        .lock()
        .unwrap()
        .pipeline
        .connect("audio-tags-changed", false, move |values| {
            let playbin = values[0]
                .get::<gstreamer::glib::Object>()
                .expect("playbin \"audio-tags-changed\" signal values[1]");

            let idx = values[1]
                .get::<i32>()
                .expect("playbin \"audio-tags-changed\" signal values[1]");

            let tags =
                playbin.emit_by_name::<Option<gstreamer::TagList>>("get-audio-tags", &[&idx]);

            if let Some(tags) = tags {
                if let Some(title) = tags.get::<gstreamer::tags::Title>() {
                    window.emit("title_event", title.get()).unwrap();
                }
            }

            None
        });
}

#[tauri::command]
fn play_station(state: tauri::State<RadioState>, station: Station) {
    let _ = state
        .player
        .lock()
        .expect("no player found")
        .play_station(station);
}

fn main() -> Result<()> {
    let state = RadioState {
        tuner: Mutex::new(Tuner::new()?),
        player: Mutex::new(Player::new()),
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

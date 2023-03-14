use crate::{
    tuner::{Station, StationsSearchQuery},
    RadioState,
};
use gstreamer::prelude::ObjectExt;
use tauri::{State, Window};

#[tauri::command]
pub fn search_stations(
    state: State<RadioState>,
    stations_query: StationsSearchQuery,
) -> Vec<Station> {
    // println!("search_stations");
    if let Ok(mut stations) = state
        .tuner
        .lock()
        .expect("no tuner found")
        .search(stations_query)
    {
        let stations_uuid: Vec<_> = stations
            .iter()
            .map(|station| station.stationuuid.clone())
            .collect();
        let bookmarked_stations_uuid = state.db.lock().unwrap().bookmark_stations_for_stationuuid_list(stations_uuid);
        if let Ok(bookmarked_station) = bookmarked_stations_uuid {
            for station in stations.iter_mut() {
                station.bookmarked = bookmarked_station.contains(&station.stationuuid);
            }
        }
        return stations.to_vec();
    }
    vec![]
}

#[tauri::command]
pub fn play_station(state: State<RadioState>, station: Station) {
    let _ = state
        .player
        .lock()
        .expect("no player found")
        .play_station(station);
}

#[tauri::command]
pub fn pause(state: State<RadioState>) {
    let _ = state.player.lock().expect("no player found").pause();
}

#[tauri::command]
pub fn play(state: State<RadioState>) {
    let _ = state.player.lock().expect("no player found").play();
}

#[tauri::command]
pub fn stream_events(state: State<RadioState>, window: Window) {
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
pub fn bookmark_station(state: State<RadioState>, station: Station) {
    let _ = state.db.lock().unwrap().bookmark_station(station);
}

#[tauri::command]
pub fn bookmark_stations_list(state: State<RadioState>) -> Vec<Station> {
    if let Ok(stations) = state.db.lock().unwrap().bookmark_stations_list() {
        return stations;
    }
    vec![]
}

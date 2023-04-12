use crate::{
    tuner::{Station, StationsSearchQuery},
    RadioState, musicbrainz::MBArtist, fanarttv::FATVArtistImages,
};
use gstreamer::{prelude::Continue, traits::ElementExt, MessageView};
use serde::Serialize;
use tauri::{State, Window};
use log::{info,debug,trace,error};

#[tauri::command]
pub fn search_stations(
    state: State<RadioState>,
    stations_query: StationsSearchQuery,
) -> Vec<Station> {
    info!("Search stations");
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
        let bookmarked_stations_uuid = state
            .db
            .lock()
            .unwrap()
            .bookmark_stations_for_stationuuid_list(stations_uuid);
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
    info!("Play station");
    let _ = state
        .player
        .lock()
        .expect("no player found")
        .play_station(station);
}

#[tauri::command]
pub fn pause(state: State<RadioState>) {
    info!("Pause");
    let _ = state.player.lock().expect("no player found").pause();
}

#[tauri::command]
pub fn play(state: State<RadioState>) {
    info!("Play");
    let _ = state.player.lock().expect("no player found").play();
}

#[tauri::command]
pub fn stream_events(state: State<RadioState>, window: Window) {
    info!("Stream events");
    let _ = state
        .player
        .lock()
        .unwrap()
        .pipeline
        .bus()
        .unwrap()
        .add_watch(move |_, message| {
            trace!("message : {:?}", message);
            match message.view() {
                MessageView::Tag(tag) => {
                    if let Some(t) = tag.tags().get::<gstreamer::tags::Title>() {
                        debug!("EVENT Title: {}",t.get());
                        window.emit("title_event", t.get()).unwrap();
                    }
                }
                MessageView::Element(element) => {
                    if let Some(structure) = element.structure() {
                        if structure.name() == "spectrum" {
                            let magnitude = structure.get::<gstreamer::List>("magnitude").unwrap();
                            let m:Vec<_> = magnitude.iter().map(|db| {
                                db.get::<f32>().unwrap_or(0.0)
                            }).collect();
                            trace!("EVENT Spectrum: {:?}", &m);
                            window.emit("spectrum_event", m).unwrap();
                        }
                    }
                }
                _ => (),
            }
            Continue(true)
        });
}

#[tauri::command]
pub fn bookmark_station(state: State<RadioState>, station: Station) {
    info!("Bookmark station");
    let _ = state.db.lock().unwrap().bookmark_station(station);
}

#[tauri::command]
pub fn remove_bookmark_station(state: State<RadioState>, station: Station) {
    info!("Remove bookmark");
    let _ = state.db.lock().unwrap().remove_bookmark_station(station);
}

#[tauri::command]
pub fn bookmark_stations_list(state: State<RadioState>) -> Vec<Station> {
    info!("Bookmarked stations list");
    if let Ok(stations) = state.db.lock().unwrap().bookmark_stations_list() {
        return stations;
    }
    vec![]
}

#[tauri::command]
pub fn set_volume(state: State<RadioState>, volume: f64) {
    info!("Set volume to {}", volume);
    let _ = state.player.lock().unwrap().set_volume(volume);
}

#[tauri::command]
pub fn mute(state: State<RadioState>, mute: bool) {
    let _ = state.player.lock().unwrap().mute(mute);
}

#[derive(Debug, Clone, Serialize)]
pub struct ArtistInfo {
    artist: Option<MBArtist>,
    images: Option<FATVArtistImages>
}

#[tauri::command]
pub fn artist_info(state: State<RadioState>, artist: String) -> Option<ArtistInfo> {
    info!("Get artist info for {}", artist);

    match state.mb.lock().unwrap().artist_info(artist) {
        Ok(info) => {
            if let Some(ref artist_infos) = info {
                match state.fatv.lock().unwrap().get_artist_images(artist_infos.id.clone()) {
                    Ok(images) => return Some(ArtistInfo{artist: info, images:Some(images)}),
                    Err(err) => error!("{}",err),
                }
            }
        },
        Err(err) => error!("Error while artist info lookup {}",err),
    }

    None
}

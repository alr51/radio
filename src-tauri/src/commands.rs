use crate::{
    fanarttv::FATVArtistImages,
    musicbrainz::MBArtist,
    tuner::{Station, StationsSearchQuery},
    RadioState,
};
use gstreamer::{prelude::Continue, traits::ElementExt, MessageView};
use log::{debug, error, info, trace};
use serde::{Deserialize, Serialize};
use tauri::{State, Window};

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
    let mut current_title = "".to_string();
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
                        let title: String = t.get().to_string();
                        debug!("EVENT Title: {}", &title);
                        if title.ne(&current_title) {
                            debug!("title change Emit event");
                            window.emit("title_event", &title).unwrap();
                            current_title = title;                            
                        }
                    }
                }
                MessageView::Element(element) => {
                    if let Some(structure) = element.structure() {
                        if structure.name() == "spectrum" {
                            let magnitude = structure.get::<gstreamer::List>("magnitude").unwrap();
                            let m: Vec<_> = magnitude
                                .iter()
                                .map(|db| db.get::<f32>().unwrap_or(0.0))
                                .collect();
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistInfo {
    pub artist: Option<MBArtist>,
    pub images: Option<FATVArtistImages>,
    pub bio: Option<String>,
}

#[tauri::command]
pub fn artist_info(state: State<RadioState>, artist: String) -> Option<ArtistInfo> {
    info!("Get artist info for {}", artist);

    // Try to retreive from cache
    if let Ok(artist_info) = state.db.lock().unwrap().get_artist_cache(artist.clone()) {
        debug!("Artist infos found in cache");
        return Some(artist_info);
    }

    // Search artist information from musicbrainz
    match state.mb.lock().unwrap().artist_info(artist.clone()) {
        Ok(info) => {
            if let Some(ref artist_infos) = info {
                // Search for wikidata url
                // And try to retrieve artist bio information
                let mut artist_bio: Option<String> = None;

                if let Some(ref relations) = artist_infos.relations {
                    let wikidata = relations.iter().find_map(|rel| {
                        if rel.url_type.eq(&Some("wikidata".to_string())) {
                            return Some(rel);
                        }
                        None
                    });

                    if let Some(wiki) = wikidata {
                        let url = wiki.clone().url;
                        if let Some(wiki_url) = url {
                            artist_bio = state
                                .wiki
                                .lock()
                                .unwrap()
                                .get_artist_extract(wiki_url.resource.unwrap(), None);
                        }
                    }
                }

                // Search for images on fanart.tv
                match state
                    .fatv
                    .lock()
                    .unwrap()
                    .get_artist_images(artist_infos.id.clone())
                {
                    Ok(images) => {
                        let artist_info = ArtistInfo {
                            artist: info,
                            images: Some(images),
                            bio: artist_bio,
                        };

                        // Add to cache
                        let _ = state.db.lock().unwrap().add_artist_cache(artist, artist_info.clone());

                        return Some(artist_info);
                    }
                    Err(err) => error!("{}", err),
                }
            }
        }
        Err(err) => error!("Error while artist info lookup {}", err),
    }

    None
}

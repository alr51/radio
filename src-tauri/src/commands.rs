use crate::{
    fanarttv::{FATVArtistImages, FanArtTv},
    musicbrainz::{MBArtist, MBReleases},
    tuner::{Station, StationsSearchQuery},
    wikipedia::Wikipedia,
    RadioState,
};
use gstreamer::{prelude::Continue, traits::ElementExt, MessageView};
use log::{debug, info, trace};
use serde::{Deserialize, Serialize};
use tauri::{State, Window};

#[tauri::command]
pub async fn search_stations(
    state: State<'_, RadioState>,
    stations_query: StationsSearchQuery,
) -> Result<Vec<Station>, ()> {
    info!("Search stations");
    if let Ok(mut stations) = state.tuner.lock().await.search(stations_query).await {
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
        return Ok(stations.to_vec());
    }
    Ok(vec![])
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
pub async fn artist_info(
    state: State<'_, RadioState>,
    artist: String,
) -> Result<Option<ArtistInfo>, ()> {
    info!("Get artist info for {}", artist);

    // Try to retreive from cache
    if let Ok(artist_info) = state.db.lock().unwrap().get_artist_cache(artist.clone()) {
        debug!("Artist infos found in cache");
        return Ok(Some(artist_info));
    }
    // Search artist information from musicbrainz
    let mb_service = state.mb.lock().await;
    let fatv_service = state.fatv.lock().await;
    let wiki_service = state.wiki.lock().await;

    if let Ok(Some(artist_info)) = mb_service.artist_info(artist.clone()).await {
        // Get wikipedia & images for artist
        let (bio, images) = tokio::join!(
            get_artist_wikipedia_data(&artist_info, wiki_service),
            get_artist_images(&artist_info, fatv_service)
        );

        let artist_data = ArtistInfo {
            artist: Some(artist_info),
            images,
            bio,
        };

        // Add artist data to cache
        let _ = state
            .db
            .lock()
            .unwrap()
            .add_artist_cache(artist, artist_data.clone());

        return Ok(Some(artist_data));
    }

    Ok(None)
}

async fn get_artist_images(
    artist_infos: &MBArtist,
    fatv_service: tokio::sync::MutexGuard<'_, FanArtTv>,
) -> Option<FATVArtistImages> {
    fatv_service
        .get_artist_images(artist_infos.id.clone())
        .await
        .ok()
}

async fn get_artist_wikipedia_data(
    artist_infos: &MBArtist,
    wiki_service: tokio::sync::MutexGuard<'_, Wikipedia>,
) -> Option<String> {
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
                return wiki_service
                    .get_artist_extract(wiki_url.resource.unwrap(), None)
                    .await;
            }
        }
    }
    None
}

#[tauri::command]
pub async fn artist_releases(
    state: State<'_, RadioState>,
    artistid: String,
) -> Result<Option<MBReleases>, ()> {
    debug!("ARTIST RELEASES !!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    debug!("ARTIST ID {}", &artistid);

    let mb = state.mb.lock().await;

    if let Ok(releases) = mb.artist_release(artistid).await {
        return Ok(releases);
    }

    Ok(None)
}

use crate::APP_USER_AGENT;
use anyhow::Result;
use log::{debug, info};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};
use serde::Deserialize;
use serde::Serialize;

pub const MUSICBRAINZ_API_ENDPOINT: &str = "https://musicbrainz.org/ws/2/";

pub struct MusicBrainz {
    pub client: Client,
}

impl MusicBrainz {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        let client = ClientBuilder::new()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .build()
            .expect("Can't build musicbrainz client");

        info!("Musicbrainz client created");
        Self { client }
    }

    pub async fn artist_info(&self, artist: String) -> Result<Option<MBArtist>> {
        let artist_url = format!("{}artist", MUSICBRAINZ_API_ENDPOINT);
        debug!("{}", &artist_url);
        let response = self
            .client
            .get(&artist_url)
            .query(&[
                ("query", artist),
                ("limit", "1".to_string()),
                ("fmt", "json".to_string()),
            ])
            .send()
            .await?
            .json::<MBArtistSearchResult>()
            .await?;

        debug!("{:?}", response);

        if response.artists.len() > 0 {
            let artist_detail_url = format!("{}/{}", artist_url, response.artists[0].id);

            debug!("detail url {}", artist_detail_url);

            let artist_response = self
                .client
                .get(artist_detail_url)
                .query(&[("inc", "url-rels".to_string()), ("fmt", "json".to_string())])
                .send()
                .await;

            debug!("{:?}", artist_response);
            let artist = artist_response?.json::<MBArtist>().await?;

            debug!("{:?}", artist);

            return Ok(Some(artist));
        }

        Ok(None)
    }

    pub async fn artist_release(&self, artist_id: String) -> Result<Option<MBReleases>> {
        let releases_url = format!("{}release", MUSICBRAINZ_API_ENDPOINT);
        let response = self
            .client
            .get(releases_url)
            .query(&[
                ("fmt", "json".to_string()),
                ("artist", artist_id),
                ("status", "official".to_string()),
                ("type", "album".to_string()),
                ("offset", "0".to_string()),
                ("limit", "100".to_string()),
            ])
            .send()
            .await?
            .json::<MBReleases>()
            .await?;

        debug!("{:?}", &response);

        Ok(Some(response))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MBArtistSearchResult {
    count: u64,
    offset: u64,
    artists: Vec<MBSearchArtist>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MBSearchArtist {
    id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MBArtist {
    pub id: String,
    pub name: Option<String>,
    pub country: Option<String>,
    pub relations: Option<Vec<MBUrlRel>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MBUrlRel {
    #[serde(alias = "type")]
    pub url_type: Option<String>,
    pub url: Option<MBUrl>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MBUrl {
    pub resource: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MBReleases {
    pub releases: Option<Vec<MBRelease>>,
    #[serde(alias = "release-offset")]
    pub offset: u64,
    #[serde(alias = "release-count")]
    pub count: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MBRelease {
    pub id: String,
    pub date: Option<String>,
    pub title: Option<String>,
    pub country: Option<String>,
    #[serde(alias = "cover-art-archive")]
    pub cover_art_archive: Option<MBCovertArtArchive>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MBCovertArtArchive {
    #[serde(default)]
    pub darkened: bool,
    #[serde(default)]
    pub back: bool,
    #[serde(default)]
    pub front: bool,
    #[serde(default)]
    pub artwork: bool,
    pub count: u64,
}

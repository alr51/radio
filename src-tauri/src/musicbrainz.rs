use crate::{APP_USER_AGENT, MUSICBRAINZ_API_ENDPOINT};
use anyhow::Result;
use log::{debug,info};
use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{HeaderMap, HeaderValue},
};
use serde::Deserialize;
use serde::Serialize;

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

    pub fn artist_info(&self, artist: String) -> Result<Option<MBArtist>> {
        let artist_url = format!("{}artist", MUSICBRAINZ_API_ENDPOINT);
        debug!("{}", &artist_url);
        let response = self
            .client
            .get(&artist_url)
            .query(&[("query", artist), ("limit", "1".to_string()), ("fmt","json".to_string())])
            .send()?
            .json::<MBArtistSearchResult>()?;

        debug!("{:?}", response);

        if response.artists.len() > 0 {
            let artist_detail_url = format!("{}/{}", artist_url, response.artists[0].id);

            debug!("detail url {}", artist_detail_url);

            let artist_response = self
                .client.clone()
                .get(artist_detail_url)
                .query(&[("inc", "url-rels".to_string()), ("fmt","json".to_string())])
                .send();


            debug!("{:?}",artist_response);
            let artist = artist_response?.json::<MBArtist>()?;

            debug!("{:?}", artist);

            return Ok(Some(artist));
        }

        Ok(None)
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
    pub url:Option<MBUrl>, 
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MBUrl {
    pub resource:Option<String>
}

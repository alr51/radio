use std::env;
use anyhow::{Result, Ok};
use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use crate::APP_USER_AGENT;
use log::debug;

const FANART_TV_API_URL: &str = "http://webservice.fanart.tv/v3/music/";

pub struct FanArtTv {
    client: Client,
}

impl FanArtTv {
    pub fn new() -> Self {
        let api_key = env::var("FANART_TV_API_KEY").expect("FANART_TV_API_KEY not set");
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("api-key", HeaderValue::from_str(&api_key).unwrap());

        let client = ClientBuilder::new()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .build()
            .expect("Can't build FanArtTv client");

        Self { client }
    }

    pub fn get_artist_images(&self, mb_artitst_id: String) -> Result<FATVArtistImages> {
        let url = format!("{}{}", FANART_TV_API_URL, mb_artitst_id);
        debug!("{}",url);
        let images = self.client.get(url).send()?.json::<FATVArtistImages>()?;
        debug!("{:?}",images);
        Ok(images)
    }
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FATVArtistImages {
    artistthumb: Option<Vec<FATVImage>>,
    artistbackground: Option<Vec<FATVImage>>,
    musicbanner:Option<Vec<FATVImage>>,
    musiclogo:Option<Vec<FATVImage>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FATVImage {
    id: String,
    url: String,
    likes: String,
}

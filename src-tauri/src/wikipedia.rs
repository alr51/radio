use crate::APP_USER_AGENT;
use log::{debug, info,error};
use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{HeaderMap, HeaderValue},
};
use serde_json::Value;

pub struct Wikipedia {
    client: Client,
}

impl Wikipedia {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        let client = ClientBuilder::new()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .build()
            .expect("Can't build wikipedia client");

        info!("Wikipedia client created");

        Self { client }
    }

    pub fn get_artist_extract(
        &self,
        wikidata_url: String,
        _language: Option<String>,
    ) -> Option<String> {
        // https://www.wikidata.org/wiki/Wikidata:Data_access/fr#Data_best_practices
        // https://doc.wikimedia.org/Wikibase/master/js/rest-api/

        // wikidata url for an article https://www.wikidata.org/wiki/Q28974
        // retrieve wikipedia sitelinks from wikidata
        // https://wikidata.org/w/rest.php/wikibase/v0/entities/items/Q28974?_fields=sitelinks
        //
        // summary from wikipedia page : https://fr.wikipedia.org/api/rest_v1/page/summary/Celeste_%28chanteuse%29

        // Extract wikidata id from wikidata url
        let wikidata_id = wikidata_url.split("/").last();

        debug!("WIKIDATA ID : {:?}", wikidata_id);

        if let Some(id) = wikidata_id {
            // construct wikidata api ul
            let wikidata_api_url = format!(
                "https://wikidata.org/w/rest.php/wikibase/v0/entities/items/{}?_fields=sitelinks",
                id
            );
            
            if let Ok(response) = self.client.get(wikidata_api_url).send() {
                if let Ok(site_links_response) = response.json::<Value>() {
                    debug!("{:?}", &site_links_response);

                    if let Some(url) = site_links_response["sitelinks"]["enwiki"]["url"].as_str() {
                        debug!("wikipedia : {url}");
                        if let Some(wikipedia_title) = url.split("/").last() {
                            let wikipedia_api_url = format!("https://en.wikipedia.org/api/rest_v1/page/summary/{}",wikipedia_title);

                            if let Ok(wiki_response) = self.client.get(wikipedia_api_url).send() {
                                if let Ok(summary_response) = wiki_response.json::<Value>() {
                                    let bio = summary_response["extract"].as_str();
                                    debug!("{:?}",&bio);

                                    return bio.map(String::from);
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

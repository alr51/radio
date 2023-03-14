use anyhow::{Ok, Result};
use serde::Deserialize;
use serde::Serialize;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

pub struct Tuner {
    api_endpoint: String,
}

impl Tuner {
    pub fn new() -> Result<Self> {
        // https://www.radio-browser.info/
        // radio-browser API endpoints lookup
        let mut api_endpoints: Vec<String> = vec![];

        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())?;
        let srv_lookup_response = resolver.srv_lookup("_api._tcp.radio-browser.info")?;

        let mut servers = srv_lookup_response.iter();
        while let Some(server) = servers.next() {
            let srv_name = server.target().to_string();
            let mut host = srv_name.chars();
            host.next_back();
            api_endpoints.push(format!("https://{}", host.as_str()));
        }
        println!("Endpoints: {:?}", api_endpoints);

        let endpoint = api_endpoints.get(0).expect("No api endpoints found");
        println!("Selected endpoint : {}", &endpoint);

        Ok(Tuner {
            api_endpoint: endpoint.to_string(),
        })
    }

    pub fn search(&mut self, query:StationsSearchQuery) -> Result<Vec<Station>> {
        let stations = reqwest::blocking::get(format!(
            "{}/json/stations/byname/{}?limit={}&order=votes&reverse=true",
            self.api_endpoint,
            query.name,
            query.limit
        ))?
        .json::<Vec<Station>>()?;

        Ok(stations)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Station {
    pub stationuuid: String,
    pub name: String,
    pub url: String,
    pub url_resolved: String,
    pub homepage: String,
    pub favicon: String,
    pub tags: String,
    pub countrycode: String,
    pub codec: String,
    pub bitrate: u16,
    pub votes: u64,
    #[serde(default)]
    pub bookmarked: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StationsSearchQuery {
    pub name: String,
    pub limit: u8,
}

impl Default for StationsSearchQuery {
    fn default() -> Self {
        Self {
            name: "Jazz".to_string(),
            limit: 20,
        }
    }
}

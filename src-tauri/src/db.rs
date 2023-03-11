use anyhow::Result;
use rusqlite::Connection;

use crate::tuner::Station;

pub struct Db {
    con: Connection,
}

impl Db {
    pub fn new() -> Self {
        let con = Db::init().expect("Can't init database");
        Db { con }
    }

    fn init() -> Result<Connection> {
        let con = Connection::open("./radio.db")?;

        con.execute(
            "CREATE TABLE IF NOT EXISTS bookmarked_stations (
                stationuuid TEXT PRIMARY KEY,
                name TEXT,
                url TEXT,
                url_resolved TEXT,
                homepage TEXT,
                favicon TEXT,
                tags TEXT,
                countrycode TEXT
            )",
            (),
        )?;

        Ok(con)
    }

    pub fn bookmark_station(&self, station: Station) -> Result<()> {
        self.con.execute(
            "INSERT INTO bookmarked_stations 
                (stationuuid, name, url, url_resolved, homepage, favicon, tags, countrycode)
                VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
            (
                station.stationuuid,
                station.name,
                station.url,
                station.url_resolved,
                station.homepage,
                station.favicon,
                station.tags,
                station.countrycode,
            ),
        )?;
        Ok(())
    }

    pub fn bookmark_stations_list(&self) -> Result<Vec<Station>> {
        let mut stmt = self.con.prepare("SELECT stationuuid, name, url, url_resolved, homepage, favicon, tags, countrycode FROM bookmarked_stations")?;

        let rows = stmt.query_map([], |row| {
            Ok(Station {
                stationuuid: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                url_resolved: row.get(3)?,
                homepage: row.get(4)?,
                favicon: row.get(5)?,
                tags: row.get(6)?,
                countrycode: row.get(7)?,
                bitrate: 0,
                codec: String::from(""),
                votes: 0,
            })
        })?;

        let mut stations: Vec<Station> = vec![];
        for station in rows {
            if let Ok(station) = station {
                stations.push(station);
            }
        }

        Ok(stations)
    }
}

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
    pub fn remove_bookmark_station(&self, station: Station) -> Result<()> {
        self.con.execute(
            "DELETE FROM bookmarked_stations WHERE stationuuid = ?1",
            (station.stationuuid,),
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
                bookmarked: true,
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

    pub fn bookmark_stations_for_stationuuid_list(
        &self,
        stationuuid_list: Vec<String>,
    ) -> Result<Vec<String>> {
        if stationuuid_list.is_empty() {
            return Ok(vec![]);
        }

        let mut s = "?,".repeat(stationuuid_list.len());
        s.pop();

        let mut stmt = self.con.prepare(&format!(
            "SELECT stationuuid FROM bookmarked_stations WHERE stationuuid IN ({})",
            s
        ))?;

        let rows = stmt.query_map(rusqlite::params_from_iter(stationuuid_list), |row| {
            row.get(0)
        })?;

        let mut stations: Vec<String> = vec![];
        for station in rows {
            if let Ok(station) = station {
                stations.push(station);
            }
        }

        Ok(stations)
    }
}

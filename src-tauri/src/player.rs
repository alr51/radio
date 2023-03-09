use anyhow::{Error, Result};
use gstreamer::{prelude::ObjectExt, traits::ElementExt};

use crate::tuner::Station;

pub struct Player {
    pub pipeline: gstreamer::Element,
}

impl Player {
    pub fn new() -> Self {
        gstreamer::init().expect("Failed to initialize gstreamer");

        let pipeline =
            gstreamer::parse_launch("playbin").expect("Failed create gstreamer pipeline");


        Player { pipeline }
    }

    pub fn play_station(&mut self, station: Station) -> Result<(), Error> {
        let uri = &station.url_resolved;
        self.pipeline.set_state(gstreamer::State::Null)?;
        self.pipeline.set_property("uri", uri);
        self.pipeline.set_state(gstreamer::State::Playing)?;

        Ok(())
    }

    pub fn play(&mut self) -> Result<(), Error> {
        self.pipeline.set_state(gstreamer::State::Playing)?;
        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), Error> {
        self.pipeline.set_state(gstreamer::State::Paused)?;
        Ok(())
    }
}

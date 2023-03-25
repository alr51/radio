use anyhow::{Error, Result};
use gstreamer::{
    prelude::*,
    traits::{ElementExt, GstBinExt},
};

use crate::tuner::Station;

pub struct Player {
    pub pipeline: gstreamer::Pipeline,
}

impl Player {
    pub fn new() -> Self {
        gstreamer::init().expect("Failed to initialize gstreamer");

        let pipeline = gstreamer::parse_launch(
            "uridecodebin name=uridecodebin ! audioconvert name=audioconvert ! tee name=t ! queue ! volume name=volume ! autoaudiosink t. ! queue ! spectrum bands=128 threshold=-60 interval=80000000 ! fakesink",
        )
        .expect("failed to create gstreamer pipeline");

        let pipeline = pipeline.downcast::<gstreamer::Pipeline>().unwrap();

        // dynamically link uridecodebin element with audioconvert element
        let uridecodebin = pipeline.by_name("uridecodebin").unwrap();
        let audioconvert = pipeline.by_name("audioconvert").unwrap();
        uridecodebin.connect_pad_added(move |_, src_pad| {
            let sink_pad = audioconvert
                .static_pad("sink")
                .expect("Failed to get static sink pad from audioconvert");
            if sink_pad.is_linked() {
                return; // We are already linked. Ignoring.
            }

            let new_pad_caps = src_pad
                .current_caps()
                .expect("Failed to get caps of new pad.");
            let new_pad_struct = new_pad_caps
                .structure(0)
                .expect("Failed to get first structure of caps.");
            let new_pad_type = new_pad_struct.name();

            if new_pad_type.starts_with("audio/x-raw") {
                // check if new_pad is audio
                let _ = src_pad.link(&sink_pad);
            }
        });

        Player { pipeline }
    }

    pub fn play_station(&mut self, station: Station) -> Result<(), Error> {
        let uri = &station.url_resolved;
        self.pipeline.set_state(gstreamer::State::Null)?;
        if let Some(uridecodebin) = self.pipeline.by_name("uridecodebin") {
            uridecodebin.set_property("uri", uri);
            self.pipeline.set_state(gstreamer::State::Playing)?;
        }

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
    
    pub fn set_volume(&mut self, vol: f64) -> Result<(),Error> {
        if let Some(volume) = self.pipeline.by_name("volume") {
            volume.set_property("volume", vol);
        }
        Ok(())
    }
    pub fn mute(&mut self, mute: bool) -> Result<(),Error> {
        if let Some(volume) = self.pipeline.by_name("volume") {
            volume.set_property("mute", mute);
        }
        Ok(())
    }
}

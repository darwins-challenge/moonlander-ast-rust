//! A module for saving objects
//! 
//! Mostly this will be used for saving a game trace for later visualization, or for exchanging
//! programs between genetic workers running on different machines.

use rustc_serialize::json;
use super::data::SensorData;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::error::Error;

/// A collection of game states in a trace
pub struct GameTrace {
    states: Vec<SensorData>
}

impl GameTrace {
    pub fn new() -> GameTrace {
        GameTrace {
            // A reasonable capacity to avoid common reallocations
            states: Vec::with_capacity(200)
        }
    }

    pub fn add(&mut self, sensor_data: &SensorData) {
        self.states.push(sensor_data.clone());
    }

    pub fn save(&self, path: &Path) -> Result<(), Box<Error>> {
        let mut f = try!(File::create(path));
        let encoded = try!(json::encode(&self.states));
        try!(f.write_all(&encoded.as_bytes()));
        Ok(())
    }
}

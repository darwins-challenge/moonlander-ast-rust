//! A module for saving objects
//! 
//! Mostly this will be used for saving a game trace for later visualization, or for exchanging
//! programs between genetic workers running on different machines.

use rustc_serialize::{json,Encodable};
use super::data::SensorData;
use super::darwin::evolve::Scores;
use super::source::Source;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::error::Error;

/// A collection of game states in a trace
#[derive(Clone)]
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

    pub fn trace(&self) -> &Vec<SensorData> {
        &self.states
    }

    pub fn save(&self, w: &mut Write) -> Result<(), Box<Error>> {
        let encoded = try!(json::encode(&self.states));
        try!(w.write_all(&encoded.as_bytes()));
        Ok(())
    }

    pub fn save_file(&self, path: &Path) -> Result<(), Box<Error>> {
        let mut f = try!(File::create(path));
        self.save(&mut f)
    }

    pub fn frames(&self) -> usize {
        self.states.len()
    }
}

pub fn writeln<T: Encodable>(x: &T, w: &mut Write) -> Result<(), Box<Error>> {
    let encoded = try!(json::encode(&x));
    try!(w.write_all(&encoded.as_bytes()));
    try!(w.write(&[10]));
    Ok(())
}

pub fn save_source<P: Source>(program: &P, w: &mut Write) -> Result<(), Box<Error>> {
    // Also save source
    try!(w.write_all(&program.source().as_bytes()));
    Ok(())
}

pub fn save_source_file<P: Source>(path: &Path, program: &P) -> Result<(), Box<Error>> {
    let mut f = try!(File::create(path));
    save_source(program, &mut f)
}

pub fn save_score(score: &Scores, w: &mut Write) -> Result<(), Box<Error>> {
    let encoded = try!(json::encode(&score));
    try!(w.write_all(&encoded.as_bytes()));
    Ok(())
}

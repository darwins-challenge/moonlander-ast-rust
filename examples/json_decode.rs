extern crate ast;
extern crate rustc_serialize;

use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use rustc_serialize::json;
use ast::structure::{Program,Command};
use ast::data::{SensorData,Evaluate};

fn main() {
    let path = Path::new("program.json");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_)    => print!("read contents of {}: {}\n", display, content),
    };

    let program: Program = json::decode(&content).unwrap();
	  let data: SensorData = SensorData { x: 37.0, y: 51.0, vx: 1.0, vy: 0.0, o: 0.0, w: 0.0, fuel: 1.0, crashed: false, landed: false, thrusting: false };

	  let command = program.evaluate(data);

	  let message = match **command {
		    Command::Skip   => "skip",
		    Command::Left   => "left",
		    Command::Right  => "right",
		    Command::Thrust => "thrust",
	  };

	  println!("message is {}", message);
}

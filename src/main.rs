#[macro_use]
extern crate ast;
extern crate rand;
extern crate rustc_serialize;

use rustc_serialize::json;

use ast::structure::{Program,Command};
use ast::data::{SensorData,Evaluate};
use ast::source::Source;

fn main() {
	let program: Program = iff!(less!(vx!(),multiply!(constant!(2.0000),constant!(3.0000))),left!(),right!());

	let data: SensorData = SensorData { x: 37.0, y: 51.0, vx: 1.0, vy: 0.0, o: 0.0, w: 0.0 };

	let command = program.evaluate(data);

	let message = match **command {
		Command::Skip  => "skip",
		Command::Left  => "turnLeft",
		Command::Right => "turnRight",
		Command::Up    => "thruster",
	};

	println!("{}", program.source());
	println!("{}", json::encode(&program).unwrap());
	println!("action is {}", message);

	// let generated: Program = rand::random();
	// println!("{}", generated.source());
}

#[macro_use]
extern crate ast;
extern crate rand;

use ast::Evaluate;

fn main() {
	let program: ast::Program = iff!(less!(vx!(), multiply!(constant!(2.0), constant!(3.0))), left!(), right!());

	let data: ast::SensorData = ast::SensorData { x: 37.0, y: 51.0, vx: 1.0, vy: 0.0, o: 0.0, w: 0.0 };

	let command = program.evaluate(data);

	let message = match **command {
		ast::Command::Skip  => "skip",
		ast::Command::Left  => "turnLeft",
		ast::Command::Right => "turnRight",
		ast::Command::Up    => "thruster",
	};

	println!("action is {}", message);

	let option: bool = rand::random();
	println!("option is {}", option);
}

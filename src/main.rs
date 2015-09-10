extern crate ast;

use ast::Evaluate;

use ast::Program::{If,Command};
use ast::Condition::Less;
use ast::Expression::{Sensor,Multiply,Constant};

fn main() {
	let program: ast::Program = If(
			Less(
			Box::new(Sensor(ast::Sensor::Vx)),
			Box::new(Multiply(
				Box::new(Constant(2.0)), 
				Box::new(Constant(3.0))
			))
		), 
		Box::new(Command(ast::Command::Left)),
		Box::new(Command(ast::Command::Right))
	);

	let data: ast::SensorData = ast::SensorData { x: 37.0, y: 51.0, vx: 6.0, vy: 0.0, o: 0.0, w: 0.0 };

	let command = program.evaluate(data);

	let message = match **command {
		ast::Command::Skip  => "skip",
		ast::Command::Left  => "turnLeft",
		ast::Command::Right => "turnRight",
		ast::Command::Up    => "thruster",
	};

	println!("action is {}", message);
}

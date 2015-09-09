extern crate ast;

use ast::Evaluate;

fn main() {
	let program: ast::Program = ast::Program::If(
		ast::Condition::Less(
			Box::new(ast::Expression::Constant(5.0)),
			Box::new(ast::Expression::Multiply(
				Box::new(ast::Expression::Constant(2.0)), 
				Box::new(ast::Expression::Constant(3.0))
			))
		), 
		Box::new(ast::Program::Command(ast::Command::Left)),
		Box::new(ast::Program::Command(ast::Command::Right))
	);

	let data: ast::SensorData = ast::SensorData { x: 37.0, y: 51.0, vx: 0.0, vy: 0.0, o: 0.0, w: 0.0 };

	let command = program.evaluate(data);

	let message = match **command {
		ast::Command::Skip  => "skip",
		ast::Command::Left  => "turnLeft",
		ast::Command::Right => "turnRight",
		ast::Command::Up    => "thruster",
	};

	println!("action is {}", message);
}

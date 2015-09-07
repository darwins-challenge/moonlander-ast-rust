extern crate ast;

use ast::Evaluate;

fn main() {
	let program: ast::Program = ast::Program::Command(ast::Command::Left);

	let command = program.evaluate();

	let message = match *command {
		ast::Command::Skip  => "skip",
		ast::Command::Left  => "turnLeft",
		ast::Command::Right => "turnRight",
		ast::Command::Up    => "thruster",
	};

	println!("action is {}", message);
}

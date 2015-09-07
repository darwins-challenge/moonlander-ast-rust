extern crate ast;

fn main() {
	let program: ast::Program = ast::Program::Command;

	match program {
		ast::Program::Command => println!("Found an action")
	}
}

extern crate ast;

fn main() {
	let program: ast::Program = ast::Program::Command(ast::Command::Skip);

	match program {
		ast::Program::Command(_) => println!("Found a command")
	}
}

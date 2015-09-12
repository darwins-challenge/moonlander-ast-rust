# moonlander-ast-rust
Code to create, manipulate and evaluate abstract syntax tree for the moonlander control code in Rust

## Usage

```rust
#[macro_use]
extern crate ast;

use ast::Evaluate;

fn main() {
	let program: ast::Program = iff!(less!(vx!(), multiply!(constant!(2.0), constant!(3.0))), left!(), right!());

	let data: ast::SensorData = ast::SensorData { x: 37.0, y: 51.0, vx: 1.0, vy: 0.0, o: 0.0, w: 0.0 };

	let command = program.evaluate(data);

	let message = match **command {
		ast::Command::Skip  => "you skipped this turn",
		ast::Command::Left  => "you turned left",
		ast::Command::Right => "you turned rigth",
		ast::Command::Up    => "you fired your thrusters",
	};

	println!("action is {}", message);
}
```

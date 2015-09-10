extern crate ast;

use ast::Evaluate;

use ast::Program::{If,Command};
use ast::Condition::{True,False,Not,Or,And,Less,LessEqual,Equal,GreaterEqual,Greater};
use ast::Expression::{Sensor,Multiply,Constant};

macro_rules! iff {
	($condition: expr, $left: expr, $right: expr) => (If($condition, Box::new($left), Box::new($right)))
}

macro_rules! T {
	() => (True)
}

macro_rules! F {
	() => (False)
}

macro_rules! not {
	($condition: expr) => (Not($condition))
}

macro_rules! or {
	($left: expr, $right: expr) => (Or(Box::new($left), Box::new($right)))
}

macro_rules! and {
	($left: expr, $right: expr) => (And(Box::new($left), Box::new($right)))
}

macro_rules! less {
	($left: expr, $right: expr) => (Less(Box::new($left), Box::new($right)))
}

macro_rules! less_equal {
	($left: expr, $right: expr) => (LessEqual(Box::new($left), Box::new($right)))
}

macro_rules! equal {
	($left: expr, $right: expr) => (Equal(Box::new($left), Box::new($right)))
}

macro_rules! greater_equal {
	($left: expr, $right: expr) => (GreaterEqual(Box::new($left), Box::new($right)))
}
macro_rules! greater {
	($left: expr, $right: expr) => (Greater(Box::new($left), Box::new($right)))
}

macro_rules! constant {
	($value: expr) => (Constant($value))
}

macro_rules! x {
	() => (Sensor(ast::Sensor::X))
}

macro_rules! y {
	() => (Sensor(ast::Sensor::Y))
}

macro_rules! vx {
	() => (Sensor(ast::Sensor::Vx))
}

macro_rules! vy {
	() => (Sensor(ast::Sensor::Vy))
}

macro_rules! o {
	() => (Sensor(ast::Sensor::O))
}

macro_rules! w {
	() => (Sensor(ast::Sensor::ws))
}

macro_rules! plus {
	($left: expr, $right: expr) => (Plus(Box::new($left), Box::new($right)))
}

macro_rules! minus {
	($left: expr, $right: expr) => (Minus(Box::new($left), Box::new($right)))
}

macro_rules! multiply {
	($left: expr, $right: expr) => (Multiply(Box::new($left), Box::new($right)))
}

macro_rules! divide {
	($left: expr, $right: expr) => (Divide(Box::new($left), Box::new($right)))
}

macro_rules! skip {
	() => (Command(ast::Command::Right));
}

macro_rules! left {
	() => (Command(ast::Command::Left))
}

macro_rules! right {
	() => (Command(ast::Command::Right))
}

macro_rules! up {
	() => (Command(ast::Command::Up))
}

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
}

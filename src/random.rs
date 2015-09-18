extern crate rand;

use super::structure::{Program,Condition,Command,Expression,Sensor};

impl rand::Rand for Program {
	fn rand<R: rand::Rng>(rng: &mut R) -> Self {
		match rng.gen_range(0, 2) {
			0 => Program::If(Condition::rand(rng), Box::new(Program::rand(rng)), Box::new(Program::rand(rng))),
			1 => Program::Command(Command::rand(rng)),
			_ => panic!(),
		}
	}
}

impl rand::Rand for Condition {
	fn rand<R: rand::Rng>(rng: &mut R) -> Self {
		match rng.gen_range(0, 10) {
			0 => Condition::True,
			1 => Condition::False,
			2 => Condition::Not(Box::new(Condition::rand(rng))),
			3 => Condition::Or(Box::new(Condition::rand(rng)), Box::new(Condition::rand(rng))),
			4 => Condition::And(Box::new(Condition::rand(rng)), Box::new(Condition::rand(rng))),
			5 => Condition::Less(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			6 => Condition::LessEqual(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			7 => Condition::Equal(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			8 => Condition::GreaterEqual(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			9 => Condition::Greater(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			_ => panic!(),
		}
	}
}

impl rand::Rand for Command {
	fn rand<R: rand::Rng>(rng: &mut R) -> Self {
		match rng.gen_range(0, 4) {
			0 => Command::Skip,
			1 => Command::Left,
			2 => Command::Right,
			3 => Command::Up,
			_ => panic!(),
		}
	}
}

impl rand::Rand for Expression {
	fn rand<R: rand::Rng>(rng: &mut R) -> Self {
		match rng.gen_range(0, 6) {
			0 => Expression::Constant(rng.next_f32()),
			1 => Expression::Sensor(Sensor::rand(rng)),
			2 => Expression::Plus(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			3 => Expression::Minus(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			4 => Expression::Multiply(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			5 => Expression::Divide(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			_ => panic!(),
		}
	}
}

impl rand::Rand for Sensor {
	fn rand<R: rand::Rng>(rng: &mut R) -> Self {
		match rng.gen_range(0, 6) {
			0 => Sensor::X,
			1 => Sensor::Y,
			2 => Sensor::Vx,
			3 => Sensor::Vy,
			4 => Sensor::O,
			5 => Sensor::W,
			_ => panic!(),
		}
	}
}
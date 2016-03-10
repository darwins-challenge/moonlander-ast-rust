extern crate rand;

pub use self::rand::Rng;

use super::structure::{Program,Condition,Command,Expression,Sensor};

#[macro_export]
macro_rules! pickWeighted {
    ($total: expr, $( $lower: expr, $upper: expr, $expression: expr),+) => {{
        let mut rng = rand::thread_rng();
		    match rng.gen_range(0, $total) {
            $( i @ $lower...$upper if i < $upper => $expression,)+
			      _ => panic!(),
		    }
    }}
}

impl rand::Rand for Program {
	fn rand<R: rand::Rng>(rng: &mut R) -> Self {
		match rng.gen_range(0, 2) {
			i @ 0...1 if i < 1 => Program::If(Condition::rand(rng), Box::new(Program::rand(rng)), Box::new(Program::rand(rng))),
			i @ 1...2 if i < 2 => Program::Command(Command::rand(rng)),
			_ => panic!(),
		}
	}
}

impl rand::Rand for Condition {
	fn rand<R: rand::Rng>(rng: &mut R) -> Self {
		match rng.gen_range(0, 10) {
			i @ 0...1  if i < 1  => Condition::True,
			i @ 1...2  if i < 2  => Condition::False,
			i @ 2...3  if i < 3  => Condition::Not(Box::new(Condition::rand(rng))),
			i @ 3...4  if i < 4  => Condition::Or(Box::new(Condition::rand(rng)), Box::new(Condition::rand(rng))),
			i @ 4...5  if i < 5  => Condition::And(Box::new(Condition::rand(rng)), Box::new(Condition::rand(rng))),
			i @ 5...6  if i < 6  => Condition::Less(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			i @ 6...7  if i < 7  => Condition::LessEqual(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			i @ 7...8  if i < 8  => Condition::Equal(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			i @ 8...9  if i < 9  => Condition::GreaterEqual(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			i @ 9...10 if i < 10 => Condition::Greater(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			_ => panic!(),
		}
	}
}

impl rand::Rand for Command {
	fn rand<R: rand::Rng>(rng: &mut R) -> Self {
		match rng.gen_range(0, 4) {
			i @ 0...1 if i < 1 => Command::Skip,
			i @ 1...2 if i < 2 => Command::Left,
			i @ 2...3 if i < 3 => Command::Right,
			i @ 3...4 if i < 4 => Command::Up,
			_ => panic!(),
		}
	}
}

impl rand::Rand for Expression {
	fn rand<R: rand::Rng>(rng: &mut R) -> Self {
		match rng.gen_range(0, 6) {
			i @ 0...1 if i < 1 => Expression::Constant(rng.next_f32()),
			i @ 1...2 if i < 2 => Expression::Sensor(Sensor::rand(rng)),
			i @ 2...3 if i < 3 => Expression::Plus(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			i @ 3...4 if i < 4 => Expression::Minus(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			i @ 4...5 if i < 5 => Expression::Multiply(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			i @ 5...6 if i < 6 => Expression::Divide(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
			_ => panic!(),
		}
	}
}

impl rand::Rand for Sensor {
	fn rand<R: rand::Rng>(rng: &mut R) -> Self {
		match rng.gen_range(0, 6) {
			i @ 0...1 if i < 1 => Sensor::X,
			i @ 1...2 if i < 2 => Sensor::Y,
			i @ 2...3 if i < 3 => Sensor::Vx,
			i @ 3...4 if i < 4 => Sensor::Vy,
			i @ 4...5 if i < 5 => Sensor::O,
			i @ 5...6 if i < 6 => Sensor::W,
			_ => panic!(),
		}
	}
}

//! The `random` module creates random `ast::structure::Program`.
//!
//! It implements `rand::Rand` for all important structures. It does this by
//! making use of a macro `pick` that can choose an expression according to
//! weight.
//!
//! # Examples
//!
//! ```
//! #[macro_use]
//! extern crate ast;
//! extern crate rand;
//! extern crate rustc_serialize;
//!
//! pub use rand::Rng;
//!
//! use ast::structure::Program;
//! use ast::source::Source;
//!
//! fn main() {
//!     let generated: Program = rand::random();
//!     println!("{}", generated.source());
//! }
//! ```

extern crate rand;

pub use self::rand::Rng;

use super::structure::{Program,Condition,Command,Expression,Sensor};

impl rand::Rand for Program {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        pick![
            3, Program::If(Box::new(Condition::rand(rng)), Box::new(Program::rand(rng)), Box::new(Program::rand(rng))),
            // This weight needs to be SLIGHTLY higher than the previous one, to increase the
            // chances of termination during random generation. Otherwise there's a too high chance
            // we're going to be generating Programs that contain Programs that contain Programs,
            // etc.
            4, Program::Command(Box::new(Command::rand(rng)))
        ]
    }
}

impl rand::Rand for Condition {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        pick![
            10, Condition::True,
            10, Condition::False,

            3,  Condition::Not(Box::new(Condition::rand(rng))),
            2,  Condition::Or(Box::new(Condition::rand(rng)), Box::new(Condition::rand(rng))),
            2,  Condition::And(Box::new(Condition::rand(rng)), Box::new(Condition::rand(rng))),

            1,  Condition::Less(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
            1,  Condition::LessEqual(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
            1,  Condition::Greater(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
            1,  Condition::GreaterEqual(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
            2,  Condition::Equal(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng)))
        ]
    }
}

impl rand::Rand for Command {
    fn rand<R: rand::Rng>(_: &mut R) -> Self {
        pick![1, Command::Skip, 1, Command::Left, 1, Command::Right, 1, Command::Thrust]
    }
}

impl rand::Rand for Expression {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        pick![
            5, Expression::Constant(rng.next_f32()),
            5, Expression::Sensor(Box::new(Sensor::rand(rng))),
            1, Expression::Plus(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
            1, Expression::Minus(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
            1, Expression::Multiply(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
            1, Expression::Divide(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng)))
        ]
    }
}

impl rand::Rand for Sensor {
    fn rand<R: rand::Rng>(_: &mut R) -> Self {
        pick![
            1, Sensor::Y,
            1, Sensor::Vy,
            1, Sensor::Fuel,
            1, Sensor::X,
            1, Sensor::Vx,
            1, Sensor::O,
            1, Sensor::W
            ]
	}
}

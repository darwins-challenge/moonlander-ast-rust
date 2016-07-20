//! Calculate the depth of an expression tree
//!
//! Mostly like copy.rs, haven't found the right abstraction yet :).

use super::structure::{Program,Condition,Command,Expression,Sensor};
use std::cmp::max;

pub trait Depth {
    fn depth(&self) -> u32;
}

impl Depth for Program {
    fn depth(&self) -> u32 {
		1 + match *self {
			Program::If(ref condition, ref left, ref right) => max(condition.depth(), max(left.depth(), right.depth())),
			Program::Command(ref command) => command.depth()
		}
    }
}

impl Depth for Condition {
    fn depth(&self) -> u32 {
		1 + match *self {
			Condition::Not(ref condition)                => condition.depth(),
			Condition::Or(ref left, ref right)           => max(left.depth(), right.depth()),
			Condition::And(ref left, ref right)          => max(left.depth(), right.depth()),
			Condition::Less(ref left, ref right)         => max(left.depth(), right.depth()),
			Condition::LessEqual(ref left, ref right)    => max(left.depth(), right.depth()),
			Condition::Equal(ref left, ref right)        => max(left.depth(), right.depth()),
			Condition::GreaterEqual(ref left, ref right) => max(left.depth(), right.depth()),
			Condition::Greater(ref left, ref right)      => max(left.depth(), right.depth()),
            _ => 0
		}
    }
}

impl Depth for Expression {
    fn depth(&self) -> u32 {
		1 + match *self {
			Expression::Sensor(ref sensor)            => sensor.depth(),
			Expression::Plus(ref left, ref right)     => max(left.depth(), right.depth()),
			Expression::Minus(ref left, ref right)    => max(left.depth(), right.depth()),
			Expression::Multiply(ref left, ref right) => max(left.depth(), right.depth()),
			Expression::Divide(ref left, ref right)   => max(left.depth(), right.depth()),
            _ => 0
		}
    }
}

impl Depth for Command {
    fn depth(&self) -> u32 {
        1
    }
}

impl Depth for Sensor {
    fn depth(&self) -> u32 {
        1
    }
}

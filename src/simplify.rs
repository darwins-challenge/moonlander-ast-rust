//! Module to simplify expression trees
use super::structure::{Program,Condition,Command,Expression,Sensor};

/// Representation of an `ast::structure::Program`
pub trait Simplify {
	fn simplify(&self) -> Self;
}

impl Simplify for Program {
	fn simplify(&self) -> Self {
		match *self {
			Program::If(ref condition, ref left, ref right) => {
                let c = condition.simplify();
                match c {
                    Condition::True => left.simplify(),
                    Condition::False => right.simplify(),
                    _ => Program::If(Box::new(c), Box::new(left.simplify()), Box::new(right.simplify()))
                }
            },
			Program::Command(_) => self.clone()
		}
	}
}

impl Simplify for Condition {
	fn simplify(&self) -> Self {
		match *self {
            Condition::Not(ref inner) => {
                match **inner {
                    Condition::Not(ref x) => x.simplify(),
                    _ => self.clone()
                }
            },
            Condition::Or(ref left, ref right) => {
                let l = left.simplify();
                let r = right.simplify();
                if l == Condition::False {
                    r
                } else if r == Condition::False {
                    l
                } else {
                    Condition::Or(Box::new(l), Box::new(r))
                }
            },
            Condition::And(ref left, ref right) => {
                let l = left.simplify();
                let r = right.simplify();
                if l == Condition::True {
                    r
                } else if r == Condition::True {
                    l
                } else {
                    Condition::And(Box::new(l), Box::new(r))
                }
            },
            _ => self.clone()
		}
	}
}

impl Simplify for Expression {
	fn simplify(&self) -> Self {
        self.clone()
	}
}

impl Simplify for Sensor {
    fn simplify(&self) -> Self {
        self.clone()
    }
}

impl Simplify for Command {
    fn simplify(&self) -> Self {
        self.clone()
    }
}

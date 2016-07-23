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
                let i = inner.simplify();
                match i {
                    Condition::True => Condition::False,
                    Condition::False => Condition::True,
                    Condition::Not(x) => *x,
                    _ => Condition::Not(Box::new(i))
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
		match *self {
            Expression::Plus(ref l, ref r)     => {
                let ls = l.simplify();
                let rs = r.simplify();

                if let Expression::Constant(lc) = ls {
                    if let Expression::Constant(rc) = rs {
                        return Expression::Constant(lc + rc);
                    }
                }

                return Expression::Plus(Box::new(ls), Box::new(rs));
            },
            Expression::Minus(ref l, ref r)    => {
                let ls = l.simplify();
                let rs = r.simplify();

                if let Expression::Constant(lc) = ls {
                    if let Expression::Constant(rc) = rs {
                        return Expression::Constant(lc - rc);
                    }
                }

                return Expression::Minus(Box::new(ls), Box::new(rs));
            },
            Expression::Multiply(ref l, ref r) => {
                let ls = l.simplify();
                let rs = r.simplify();

                if let Expression::Constant(lc) = ls {
                    if let Expression::Constant(rc) = rs {
                        return Expression::Constant(lc * rc);
                    }
                }

                return Expression::Multiply(Box::new(ls), Box::new(rs));
            },
            Expression::Divide(ref l, ref r)   => {
                let ls = l.simplify();
                let rs = r.simplify();

                if let Expression::Constant(lc) = ls {
                    if let Expression::Constant(rc) = rs {
                        return Expression::Constant(lc / rc);
                    }
                }

                return Expression::Divide(Box::new(ls), Box::new(rs));
            },
            _ => self.clone()
        }
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

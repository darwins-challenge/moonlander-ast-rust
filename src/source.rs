//! The `source` module provides a representation of `ast::structure::Program`s
//!
//! It defines a trait `Source` that sources a `ast::structure::Program` to a `String`. 

use super::structure::{Program,Condition,Command,Expression,Sensor};

/// Representation of an `ast::structure::Program`
pub trait Source {
	fn source(&self) -> String;
}

impl Source for Program {
	fn source(&self) -> String {
		match *self {
			Program::If(ref condition, ref left, ref right) => format!("iff!({},{},{})", condition.source(), left.source(), right.source()),
			Program::Command(ref command)                   => command.source(),
		}
	}
}

impl Source for Condition {
	fn source(&self) -> String {
		match *self {
			Condition::True                              => format!("T!()"),
			Condition::False                             => format!("F!()"),
			Condition::Not(ref condition)                => format!("not!({})", (*condition).source()),
			Condition::Or(ref left, ref right)           => format!("or!({},{})", (*left).source(), (*right).source()),
			Condition::And(ref left, ref right)          => format!("and!({},{})", (*left).source(), (*right).source()),

			Condition::Less(ref left, ref right)         => format!("less!({},{})", (*left).source(), (*right).source()),
			Condition::LessEqual(ref left, ref right)    => format!("less_equal!({},{})", (*left).source(), (*right).source()),
			Condition::Equal(ref left, ref right)        => format!("equal!({},{})", (*left).source(), (*right).source()),
			Condition::GreaterEqual(ref left, ref right) => format!("greater_equal!({},{})", (*left).source(), (*right).source()),
			Condition::Greater(ref left, ref right)      => format!("greater!({},{})", (*left).source(), (*right).source()),
		}
	}
}

impl Source for Expression {
	fn source(&self) -> String {
		match *self {
			Expression::Constant(value)               => format!("constant!({:.*})", 4, value),
			Expression::Sensor(ref sensor)            => sensor.source(),
			Expression::Plus(ref left, ref right)     => format!("plus!({},{})", (*left).source(), (*right).source()),
			Expression::Minus(ref left, ref right)    => format!("minus!({},{})", (*left).source(), (*right).source()),
			Expression::Multiply(ref left, ref right) => format!("multiply!({},{})", (*left).source(), (*right).source()),
			Expression::Divide(ref left, ref right)   => format!("divide!({},{})", (*left).source(), (*right).source()),
		}
	}
}

impl Source for Sensor {
	fn source(&self) -> String {
		match *self {
			Sensor::X  => format!("x!()"),
			Sensor::Y  => format!("u!()"),
			Sensor::Vx => format!("vx!()"),
			Sensor::Vy => format!("vy!()"),
			Sensor::O  => format!("o!()"),
			Sensor::W  => format!("w!()"),
		}
	}
}

impl Source for Command {
	fn source(&self) -> String {
		match *self {
			Command::Skip   => format!("skip!()"),
			Command::Left   => format!("left!()"),
			Command::Right  => format!("right!()"),
			Command::Thrust => format!("thrust!()"),
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use structure::{Program,Condition,Command,Expression,Sensor};

    #[test]
    fn should_correctly_source_programs() {
	      assert_eq!("iff!(T!(),skip!(),left!())",
		               Program::If(Condition::True, Box::new(Program::Command(Command::Skip)), Box::new(Program::Command(Command::Left))).source());
	      assert_eq!("skip!()", Program::Command(Command::Skip).source());
    }

    #[test]
    fn should_correctly_source_conditions() {
	      assert_eq!("T!()", Condition::True.source());
	      assert_eq!("F!()", Condition::False.source());
	      assert_eq!("not!(T!())", Condition::Not(Box::new(Condition::True)).source());
	      assert_eq!("or!(T!(),F!())", Condition::Or(Box::new(Condition::True), Box::new(Condition::False)).source());
	      assert_eq!("and!(F!(),T!())", Condition::And(Box::new(Condition::False), Box::new(Condition::True)).source());
	      assert_eq!("less!(constant!(1.0000),constant!(2.0000))",
		               Condition::Less(Box::new(Expression::Constant(1.0)), Box::new(Expression::Constant(2.0))).source());
	      assert_eq!("less_equal!(constant!(1.0000),constant!(2.0000))",
		               Condition::LessEqual(Box::new(Expression::Constant(1.0)), Box::new(Expression::Constant(2.0))).source());
	      assert_eq!("equal!(constant!(1.0000),constant!(2.0000))",
		               Condition::Equal(Box::new(Expression::Constant(1.0)), Box::new(Expression::Constant(2.0))).source());
	      assert_eq!("greater_equal!(constant!(1.0000),constant!(2.0000))",
		               Condition::GreaterEqual(Box::new(Expression::Constant(1.0)), Box::new(Expression::Constant(2.0))).source());
	      assert_eq!("greater!(constant!(1.0000),constant!(2.0000))",
		               Condition::Greater(Box::new(Expression::Constant(1.0)), Box::new(Expression::Constant(2.0))).source());
    }

    #[test]
    fn should_correctly_source_expressions() {
	      assert_eq!("constant!(1.0000)", Expression::Constant(1.0).source());
	      assert_eq!("vx!()", Expression::Sensor(Sensor::Vx).source());
	      assert_eq!("plus!(constant!(1.0000),vy!())",
		               Expression::Plus(Box::new(Expression::Constant(1.0)),Box::new(Expression::Sensor(Sensor::Vy))).source());
	      assert_eq!("minus!(constant!(1.0000),constant!(2.0000))",
		               Expression::Minus(Box::new(Expression::Constant(1.0)),Box::new(Expression::Constant(2.0))).source());
	      assert_eq!("multiply!(constant!(1.0000),constant!(2.0000))",
		               Expression::Multiply(Box::new(Expression::Constant(1.0)),Box::new(Expression::Constant(2.0))).source());
	      assert_eq!("divide!(constant!(1.0000),constant!(2.0000))",
		               Expression::Divide(Box::new(Expression::Constant(1.0)),Box::new(Expression::Constant(2.0))).source());
    }

    #[test]
    fn should_correctly_source_sensors() {
	      assert_eq!("x!()",  Sensor::X.source());
	      assert_eq!("u!()",  Sensor::Y.source());
	      assert_eq!("vx!()", Sensor::Vx.source());
	      assert_eq!("vy!()", Sensor::Vy.source());
	      assert_eq!("o!()",  Sensor::O.source());
	      assert_eq!("w!()",  Sensor::W.source());
    }

    #[test]
    fn should_correctly_source_commands() {
	      assert_eq!("skip!()",  Command::Skip.source());
	      assert_eq!("left!()",  Command::Left.source());
	      assert_eq!("right!()", Command::Right.source());
	      assert_eq!("thrust!()",    Command::Thrust.source());
    }
}

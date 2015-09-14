use super::structure::{Program,Condition,Command,Expression,Sensor};

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
			Command::Skip  => format!("skip!()"),
			Command::Left  => format!("left!()"),
			Command::Right => format!("right!()"),
			Command::Up    => format!("up!()"),
		}
	}
}
//! The `data` model is responsible for creation and evaluation of data in the
//! moon lander domain.
//!
//! The center piece is [`ast::data::SensorData`](struct.SensorData.html) that
//! gets evaluated by a
//! [`ast::structure::Program`](../structure/enum.Program.html) to produce a
//! [`ast::structure::Command`](../structure/enum.Command.html).

use super::structure::{Program,Condition,Command,Expression,Sensor};

/// `SensorData` represents the information that is available for programs to decide what `ast::structure::Command`
/// to execute when it is evaluated.
///
/// # Examples
///
/// ```
/// let data = ast::data::SensorData { x: 37.0, y: 51.0, vx: 1.0, vy: 0.0, o: 0.0, w: 0.0 };
/// ```
#[derive(Copy,Clone)]
pub struct SensorData {
	pub x:  f32,
	pub y:  f32,
	pub vx: f32,
	pub vy: f32,
	pub o:  f32,
	pub w:  f32
}

/// `Evaluate` returns a specific `ast::structure::Command` to execute, depending on `ast::data::SensorData`
pub trait Evaluate {
	fn evaluate(&self, sensor_data: SensorData) -> Box<&Command>;
}

impl Evaluate for Program {
	fn evaluate(&self, sensor_data: SensorData) -> Box<&Command> {
		match *self {
			Program::If(ref condition, ref true_program, ref false_program) => {
				if (*condition).value(sensor_data) {
					Box::new(*true_program.evaluate(sensor_data))
				} else {
					Box::new(*false_program.evaluate(sensor_data))
				}
			},
			Program::Command(ref command) => Box::new(command),
		}
	}
}

/// The numeric value of an `ast::structure::Expression`
pub trait NumericValue {
	fn value(&self, sensor_data: SensorData) -> f32;
}

impl NumericValue for Expression {
	fn value(&self, sensor_data: SensorData) -> f32 {
		match *self {
			Expression::Constant(value)               => value,
			Expression::Sensor(ref sensor)            => sensor.value(sensor_data), 
			Expression::Plus(ref left, ref right)     => left.value(sensor_data) + right.value(sensor_data),
			Expression::Minus(ref left, ref right)    => left.value(sensor_data) - right.value(sensor_data),
			Expression::Multiply(ref left, ref right) => left.value(sensor_data) * right.value(sensor_data),
			Expression::Divide(ref left, ref right)   => left.value(sensor_data) / right.value(sensor_data)
		}
	}
}

impl NumericValue for Sensor {
	fn value(&self, sensor_data: SensorData) -> f32 {
		match *self {
			Sensor::X  => sensor_data.x,
			Sensor::Y  => sensor_data.y,
			Sensor::Vx => sensor_data.vx,
			Sensor::Vy => sensor_data.vy,
			Sensor::O  => sensor_data.o,
			Sensor::W  => sensor_data.w,
		}
	}
}

/// The truth value of an `ast::structure::Condition`
pub trait BooleanValue {
	fn value(&self, sensor_data: SensorData) -> bool;
}

impl BooleanValue for Condition {
	fn value(&self, sensor_data: SensorData) -> bool {
		match *self {
			Condition::True                              => true,
			Condition::False                             => false,
			Condition::Not(ref condition)                => !(*condition).value(sensor_data),
			Condition::Or(ref left, ref right)           => (*left).value(sensor_data) || (*right).value(sensor_data),
			Condition::And(ref left, ref right)          => (*left).value(sensor_data) && (*right).value(sensor_data),
			Condition::Less(ref left, ref right)         => (*left).value(sensor_data) <  (*right).value(sensor_data),
			Condition::LessEqual(ref left, ref right)    => (*left).value(sensor_data) <= (*right).value(sensor_data),
			Condition::Equal(ref left, ref right)        => (*left).value(sensor_data) == (*right).value(sensor_data),
			Condition::GreaterEqual(ref left, ref right) => (*left).value(sensor_data) >= (*right).value(sensor_data),
			Condition::Greater(ref left, ref right)      => (*left).value(sensor_data) >  (*right).value(sensor_data),
		}
	}
}

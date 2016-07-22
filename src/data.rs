//! The `data` model is responsible for creation and evaluation of data in the
//! moon lander domain.
//!
//! The center piece is [`ast::data::SensorData`](struct.SensorData.html) that
//! gets evaluated by a
//! [`ast::structure::Program`](../structure/enum.Program.html) to produce a
//! [`ast::structure::Command`](../structure/enum.Command.html).

use super::structure::{Program,Condition,Command,Expression,Sensor,Number};

/// `SensorData` represents the information that is available for programs to decide what `ast::structure::Command`
/// to execute when it is evaluated.
#[derive(Copy,Clone,RustcEncodable)]
pub struct SensorData {
    pub x:  Number,
    pub y:  Number,
    pub vx: Number,
    pub vy: Number,
    pub o:  Number,
    pub w:  Number,
    pub fuel: Number,
    pub hit_ground: bool,
    pub landed: bool,
    pub thrusting: bool,
    pub crash_speed: Number
}

impl SensorData {
    /// Creates a builder for `SensorData`. It should be used in the following sense:
    ///
    /// ```
    /// let sensor_data = ast::data::SensorData::new().with_y(100.0);
    /// ```
    pub fn new () -> SensorData {
        SensorData {
            x:          0.0,
            y:          0.0,
            vx:         0.0,
            vy:         0.0,
            o:          0.0,
            w:          0.0,
            fuel:       1.0,
            hit_ground: false,
            landed:     false,
            thrusting:  false,
            crash_speed: 0.0,
        }
    }

    pub fn with_x(self, x: Number) -> SensorData {
        SensorData { x: x, ..self }
    }

    pub fn with_y(self, y: Number) -> SensorData {
        SensorData { y: y, ..self }
    }

    pub fn with_vx(self, vx: Number) -> SensorData {
        SensorData { vx: vx, ..self }
    }

    pub fn with_vy(self, vy: Number) -> SensorData {
        SensorData { vy: vy, ..self }
    }

    pub fn with_o(self, o: Number) -> SensorData {
        SensorData { o: o, ..self }
    }

    pub fn with_w(self, w: Number) -> SensorData {
        SensorData { w: w, ..self }
    }

    pub fn with_fuel(self, fuel: Number) -> SensorData {
        SensorData { fuel: fuel, ..self }
    }

    pub fn hit_ground(self) -> SensorData {
        SensorData { hit_ground: true, ..self }
    }

    pub fn landed(self) -> SensorData {
        SensorData { landed: true, ..self }
    }

    pub fn thrusting(self) -> SensorData {
        SensorData { thrusting: true, ..self }
    }
}

/// `Evaluate` returns a specific `ast::structure::Command` to execute, depending on `ast::data::SensorData`
pub trait Evaluate {
	fn evaluate(&self, sensor_data: SensorData) -> Command;
}

impl Evaluate for Program {
	fn evaluate(&self, sensor_data: SensorData) -> Command {
		match *self {
			Program::If(ref condition, ref true_program, ref false_program) => {
				if (*condition).value(sensor_data) {
					true_program.evaluate(sensor_data)
				} else {
					false_program.evaluate(sensor_data)
				}
			},
			Program::Command(ref command) => **command
		}
	}
}

/// The numeric value of an `ast::structure::Expression`
pub trait NumericValue {
	fn value(&self, sensor_data: SensorData) -> Number;
}

impl NumericValue for Expression {
	fn value(&self, sensor_data: SensorData) -> Number {
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
    fn value(&self, sensor_data: SensorData) -> Number {
        match *self {
//            Sensor::X    => sensor_data.x,
            Sensor::Y    => sensor_data.y,
//            Sensor::Vx   => sensor_data.vx,
            Sensor::Vy   => sensor_data.vy,
//            Sensor::O    => sensor_data.o,
//            Sensor::W    => sensor_data.w,
            Sensor::Fuel => sensor_data.fuel,
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
			Condition::Not(ref condition)                => !condition.value(sensor_data),
			Condition::Or(ref left, ref right)           => left.value(sensor_data) || right.value(sensor_data),
			Condition::And(ref left, ref right)          => left.value(sensor_data) && right.value(sensor_data),
			Condition::Less(ref left, ref right)         => left.value(sensor_data) <  right.value(sensor_data),
			Condition::LessEqual(ref left, ref right)    => left.value(sensor_data) <= right.value(sensor_data),
			Condition::Equal(ref left, ref right)        => left.value(sensor_data) == right.value(sensor_data),
			Condition::GreaterEqual(ref left, ref right) => left.value(sensor_data) >= right.value(sensor_data),
			Condition::Greater(ref left, ref right)      => left.value(sensor_data) >  right.value(sensor_data),
		}
	}
}

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
///
/// # Examples
///
/// ```
/// let data = ast::data::SensorData { x: 37.0, y: 51.0, vx: 1.0, vy: 0.0, o: 0.0, w: 0.0, fuel: 1.0, crashed: false, landed: false, thrusting: false };
/// ```
#[derive(Copy,Clone,RustcEncodable)]
pub struct SensorData {
    pub x:  Number,
    pub y:  Number,
    pub vx: Number,
    pub vy: Number,
    pub o:  Number,
    pub w:  Number,
    pub fuel: Number,
    pub crashed: bool,
    pub landed: bool,
    pub thrusting: bool,
}

impl SensorData {
    /// Creates a builder for `SensorData`. It should be used in the following sense:
    ///
    /// ```
    /// let sensor_data = ast::data::SensorData::new().with_y(100.0).build();
    /// ```
    pub fn new () -> SensorData {
        SensorData {
            x:         0.0,
            y:         0.0,
            vx:        0.0,
            vy:        0.0,
            o:         0.0,
            w:         0.0,
            fuel:      1.0,
            crashed:   false,
            landed:    false,
            thrusting: false,
        }
    }

    pub fn with_x<'a>(&'a mut self, x: Number) -> &'a mut SensorData {
        self.x = x;
        self
    }

    pub fn with_y<'a>(&'a mut self, y: Number) -> &'a mut SensorData {
        self.y = y;
        self
    }

    pub fn with_vx<'a>(&'a mut self, vx: Number) -> &'a mut SensorData {
        self.vx = vx;
        self
    }

    pub fn with_vy<'a>(&'a mut self, vy: Number) -> &'a mut SensorData {
        self.vy = vy;
        self
    }

    pub fn with_o<'a>(&'a mut self, o: Number) -> &'a mut SensorData {
        self.o = o;
        self
    }

    pub fn with_w<'a>(&'a mut self, w: Number) -> &'a mut SensorData {
        self.w = w;
        self
    }

    pub fn with_fuel<'a>(&'a mut self, fuel: Number) -> &'a mut SensorData {
        self.fuel = fuel;
        self
    }

    pub fn crashed<'a>(&'a mut self) -> &'a mut SensorData {
        self.crashed = true;
        self
    }

    pub fn landed<'a>(&'a mut self) -> &'a mut SensorData {
        self.crashed = true;
        self
    }

    pub fn thrusting<'a>(&'a mut self) -> &'a mut SensorData {
        self.thrusting = true;
        self
    }

    pub fn build<'a>(&'a self) -> SensorData {
        *self
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
            Sensor::X    => sensor_data.x,
            Sensor::Y    => sensor_data.y,
            Sensor::Vx   => sensor_data.vx,
            Sensor::Vy   => sensor_data.vy,
            Sensor::O    => sensor_data.o,
            Sensor::W    => sensor_data.w,
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

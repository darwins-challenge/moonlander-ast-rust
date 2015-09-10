pub enum Program {
	If(Condition, Box<Program>, Box<Program>),
	Command(Command),
}


pub enum Condition {
	True,
	False,
	Not(Box<Condition>),
	Or(Box<Condition>, Box<Condition>),
	And(Box<Condition>, Box<Condition>),

	Less(Box<Expression>, Box<Expression>),
	LessEqual(Box<Expression>, Box<Expression>),
	Equal(Box<Expression>, Box<Expression>),
	GreaterEqual(Box<Expression>, Box<Expression>),
	Greater(Box<Expression>, Box<Expression>),
}

pub enum Expression {
	Constant(f32),
	Sensor(Sensor),
	Plus(Box<Expression>, Box<Expression>),
	Minus(Box<Expression>, Box<Expression>),
	Multiply(Box<Expression>, Box<Expression>),
	Divide(Box<Expression>, Box<Expression>),
}

pub enum Sensor {
	X,
	Y,
	Vx,
	Vy,
	O,
	W,
}

pub enum Command {
	Skip,
	Left,
	Right,
	Up
}

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

#[derive(Copy,Clone)]
pub struct SensorData {
	pub x: f32,
	pub y: f32,
	pub vx: f32,
	pub vy: f32,
	pub o: f32,
	pub w: f32
}

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

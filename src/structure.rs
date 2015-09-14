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
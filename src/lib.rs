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
	Plus(Box<Expression>, Box<Expression>),
	Minus(Box<Expression>, Box<Expression>),
	Multiply(Box<Expression>, Box<Expression>),
	Divide(Box<Expression>, Box<Expression>),
}

pub enum Command {
	Skip,
	Left,
	Right,
	Up
}

pub trait NumericValue {
	fn value(&self) -> f32;
}

impl NumericValue for Expression {
	fn value(&self) -> f32 {
		match *self {
			Expression::Constant(value) => value,
			Expression::Plus(ref left, ref right)     => left.value() + right.value(),
			Expression::Minus(ref left, ref right)    => left.value() - right.value(),
			Expression::Multiply(ref left, ref right) => left.value() * right.value(),
			Expression::Divide(ref left, ref right)   => left.value() / right.value()
		}
	}
}

pub trait BooleanValue {
	fn value(&self) -> bool;
}

impl BooleanValue for Condition {
	fn value(&self) -> bool {
		match *self {
			Condition::True                              => true,
			Condition::False                             => false,
			Condition::Not(ref condition)                => !(*condition).value(),
			Condition::Or(ref left, ref right)           => (*left).value() || (*right).value(),
			Condition::And(ref left, ref right)          => (*left).value() && (*right).value(),
			Condition::Less(ref left, ref right)         => (*left).value() <  (*right).value(),
			Condition::LessEqual(ref left, ref right)    => (*left).value() <= (*right).value(),
			Condition::Equal(ref left, ref right)        => (*left).value() == (*right).value(),
			Condition::GreaterEqual(ref left, ref right) => (*left).value() >= (*right).value(),
			Condition::Greater(ref left, ref right)      => (*left).value() >  (*right).value(),
		}
	}
}

pub trait Evaluate {
	fn evaluate(&self) -> Box<&Command>;
}

impl Evaluate for Program {
	fn evaluate(&self) -> Box<&Command> {
		match *self {
			Program::If(ref condition, ref true_program, ref false_program) => {
				if (*condition).value() {
					Box::new(*true_program.evaluate())
				} else {
					Box::new(*false_program.evaluate())
				}
			},
			Program::Command(ref command) => Box::new(command),
		}
	}
}

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
}

pub enum Command {
	Skip,
	Left,
	Right,
	Up
}

pub trait Evaluate {
	fn evaluate(&self) -> Box<&Command>;
}

impl Evaluate for Program {
	fn evaluate(&self) -> Box<&Command> {
		match *self {
			Program::If(ref condition, ref trueProgram, ref falseProgram) => {
				Box::new(*trueProgram.evaluate())
			},
			Program::Command(ref command) => Box::new(command),
		}
	}
}

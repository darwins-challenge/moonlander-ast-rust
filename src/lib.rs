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

pub trait Value {
	fn value(&self) -> bool;
}

impl Value for Condition {
	fn value(&self) -> bool {
		match *self {
			Condition::True                     => true,
			Condition::False                    => false,
			Condition::Not(ref condition)       => !(*condition).value(),
			Condition::Or(ref left, ref right)  => (*left).value() || (*right).value(),
			Condition::And(ref left, ref right) => (*left).value() && (*right).value(),
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

pub trait Evaluate {
	fn evaluate(&self) -> &Command;
}

pub enum Program {
	Command(Command),
}

impl Evaluate for Program {
	fn evaluate(&self) -> &Command {
		match *self {
			Program::Command(ref command) => command,
		}
	}
}

pub enum Command {
	Skip,
	Left,
	Right,
	Up
}

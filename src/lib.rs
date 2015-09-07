pub enum Program {
	Command(Command),
}

pub enum Command {
	Skip,
	Left,
	Right,
	Up
}

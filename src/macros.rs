#[macro_export]
macro_rules! iff {
	($condition: expr, $left: expr, $right: expr) => (ast::structure::Program::If($condition, Box::new($left), Box::new($right)))
}

#[macro_export]
macro_rules! T {
	() => (ast::structure::Condition::True)
}

#[macro_export]
macro_rules! F {
	() => (ast::structure::Condition::False)
}

#[macro_export]
macro_rules! not {
	($condition: expr) => (ast::structure::Condition::Not(Box::new($condition)))
}

#[macro_export]
macro_rules! or {
	($left: expr, $right: expr) => (ast::structure::Condition::Or(Box::new($left), Box::new($right)))
}

#[macro_export]
macro_rules! and {
	($left: expr, $right: expr) => (ast::structure::Condition::And(Box::new($left), Box::new($right)))
}

#[macro_export]
macro_rules! less {
	($left: expr, $right: expr) => (ast::structure::Condition::Less(Box::new($left), Box::new($right)))
}

#[macro_export]
macro_rules! less_equal {
	($left: expr, $right: expr) => (ast::structure::Condition::LessEqual(Box::new($left), Box::new($right)))
}

#[macro_export]
macro_rules! equal {
	($left: expr, $right: expr) => (ast::structure::Condition::Equal(Box::new($left), Box::new($right)))
}

#[macro_export]
macro_rules! greater_equal {
	($left: expr, $right: expr) => (ast::structure::Condition::GreaterEqual(Box::new($left), Box::new($right)))
}

#[macro_export]
macro_rules! greater {
	($left: expr, $right: expr) => (ast::structure::Condition::Greater(Box::new($left), Box::new($right)))
}

#[macro_export]
macro_rules! constant {
	($value: expr) => (ast::structure::Expression::Constant($value))
}

#[macro_export]
macro_rules! x {
	() => (ast::structure::Expression::Sensor(ast::structure::Sensor::X))
}

#[macro_export]
macro_rules! y {
	() => (ast::structure::Expression::Sensor(ast::structure::Sensor::Y))
}

#[macro_export]
macro_rules! vx {
	() => (ast::structure::Expression::Sensor(ast::structure::Sensor::Vx))
}

#[macro_export]
macro_rules! vy {
	() => (ast::structure::Expression::Sensor(ast::structure::Sensor::Vy))
}

#[macro_export]
macro_rules! o {
	() => (ast::structure::Expression::Sensor(ast::structure::Sensor::O))
}

#[macro_export]
macro_rules! w {
	() => (ast::structure::Expression::Sensor(ast::structure::Sensor::W))
}

#[macro_export]
macro_rules! plus {
	($left: expr, $right: expr) => (ast::structure::Expression::Plus(Box::new($left), Box::new($right)))
}

#[macro_export]
macro_rules! minus {
	($left: expr, $right: expr) => (ast::structure::Expression::Minus(Box::new($left), Box::new($right)))
}

#[macro_export]
macro_rules! multiply {
	($left: expr, $right: expr) => (ast::structure::Expression::Multiply(Box::new($left), Box::new($right)))
}

#[macro_export]
macro_rules! divide {
	($left: expr, $right: expr) => (ast::structure::Expression::Divide(Box::new($left), Box::new($right)))
}

#[macro_export]
macro_rules! skip {
	() => (ast::structure::Program::Command(ast::structure::Command::Skip));
}

#[macro_export]
macro_rules! left {
	() => (ast::structure::Program::Command(ast::structure::Command::Left))
}

#[macro_export]
macro_rules! right {
	() => (ast::structure::Program::Command(ast::structure::Command::Right))
}

#[macro_export]
macro_rules! thrust {
	() => (ast::structure::Program::Command(ast::structure::Command::Thrust))
}

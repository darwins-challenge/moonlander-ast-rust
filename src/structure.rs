//! The `structure` is the abstract syntax tree for the moon lander control
//! code.
//!
//! It is the hearth of the crate. Creating a `Program` by hand is a little
//! bit tedious. The tedium is a little bit relieved with the introduction of
//! [macros](../index.html#macros), the possibility to read from and write to
//! JSON and to generate a [random program](../random/index.html).

/// The `Program` enum is the root of the abstract syntax tree.
///
/// `Program` is either a single `Command` or an `If`-statement.
///
/// You can find syntactic sugar to create `Program`s more easily in [macros](../index.html#macros).
///
/// # Examples
///
/// ```
/// extern crate ast;
///
/// let program: ast::structure::Program = ast::structure::Program::If(
///	  Box::new(ast::structure::Condition::True),
///   Box::new(ast::structure::Program::Command(Box::new(ast::structure::Command::Skip))),
///   Box::new(ast::structure::Program::Command(Box::new(ast::structure::Command::Thrust)))
/// );
/// ```
///

use std::fmt;

#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq)]
pub enum Program {
	If(Box<Condition>, Box<Program>, Box<Program>),
	Command(Box<Command>),
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Program::If(ref cond, ref one, ref two) => write!(f, "({} then {} else {})", cond, one, two),
            Program::Command(ref command) => write!(f, "{}", command)
        }
    }
}

/// The general number type used by all simulations
pub type Number = f32;

/// The `Condition` enum is used as the condition in a `ast::Program::If`
///
/// The following groups are present in `Condition`:
/// **literal**: it includes `Condition::True` and `Condition::False`.
/// **logical**: it includes `Condition::Not`, `Condition::Or` and `Condition::And`.
/// **comparative**: it includes `Condition::Less`, `Condition::LessEqual`, `Condition::Equal`, `Condition::GreaterEqual`, and `Condition::Greater`
///
/// You can find syntactic sugar to create `Condition`s more easily in [macros](../index.html#macros)
///
/// # Examples
///
/// ```
/// extern crate ast;
///
/// let condition: ast::structure::Condition = ast::structure::Condition::And(
///   Box::new(ast::structure::Condition::Not(Box::new(ast::structure::Condition::False))),
///   Box::new(ast::structure::Condition::Less(
///     Box::new(ast::structure::Expression::Constant(1.0)),
///     Box::new(ast::structure::Expression::Constant(2.0))
///   ))
/// );
/// ```
#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq)]
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

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Condition::True                       => write!(f, "True"),
            Condition::False                      => write!(f, "False"),

            Condition::Not(ref c)                 => write!(f, "!{}", c),
            Condition::Or(ref l, ref r)           => write!(f, "({} || {})", l, r),
            Condition::And(ref l, ref r)          => write!(f, "({} && {})", l, r),

            Condition::Less(ref l, ref r)         => write!(f, "({} < {})", l, r),
            Condition::LessEqual(ref l, ref r)    => write!(f, "({} <= {})", l, r),
            Condition::Equal(ref l, ref r)        => write!(f, "({} == {})", l, r),
            Condition::GreaterEqual(ref l, ref r) => write!(f, "({} >= {})", l, r),
            Condition::Greater(ref l, ref r)      => write!(f, "({} > {})", l, r),
        }
    }
}

/// The `Expression` enum is used as the **comparative** `ast::structure::Condition`s
///
/// It allows you to do calculations with `Expression::Constant`s or `Expression::Sensor`s
///
/// You can find syntactic sugar to create `Expression`s more easily in [macros](../index.html#macros)
///
/// # Examples
///
/// ```
/// extern crate ast;
///
/// let expression: ast::structure::Expression = ast::structure::Expression::Plus(
///   Box::new(ast::structure::Expression::Constant(42.0)),
///   Box::new(ast::structure::Expression::Sensor(Box::new(ast::structure::Sensor::Vy)))
/// );
/// ```
#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq)]
pub enum Expression {
	  Constant(Number),
	  Sensor(Box<Sensor>),
	  Plus(Box<Expression>, Box<Expression>),
	  Minus(Box<Expression>, Box<Expression>),
	  Multiply(Box<Expression>, Box<Expression>),
	  Divide(Box<Expression>, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Constant(ref n)        => write!(f, "{}", n),
            Expression::Sensor(ref s)          => write!(f, "{}", s),
            Expression::Plus(ref l, ref r)     => write!(f, "({} + {})", l, r),
            Expression::Minus(ref l, ref r)    => write!(f, "({} - {})", l, r),
            Expression::Multiply(ref l, ref r) => write!(f, "({} * {})", l, r),
            Expression::Divide(ref l, ref r)   => write!(f, "({} / {})", l, r),
        }
    }
}

/// The `Sensor` enum is used in `ast::Structure::Expression` as input to calculations.
///
/// It is a specific part of `ast::data::SensorData`.
///
/// You can find syntactic sugar to create `ensor`s more easily in [macros](../index.html#macros)
///
/// # Examples
///
/// ```
/// extern crate ast;
///
/// let sensor: ast::structure::Sensor = ast::structure::Sensor::Vy;
/// ```
#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq,Copy)]
pub enum Sensor {
      X,
	  Y,
      Vx,
	  Vy,
      O,
      W,
      Fuel,
}

impl fmt::Display for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Sensor::Y    => write!(f, "Y"),
            Sensor::Vy   => write!(f, "Vy"),
            Sensor::Fuel => write!(f, "Fuel"),
            Sensor::X    => write!(f, "X"),
            Sensor::Vx   => write!(f, "Vx"),
            Sensor::O    => write!(f, "O"),
            Sensor::W    => write!(f, "W")
        }
    }
}

/// The `Command` enum is used as an argument to `ast::structure::Program::Command`
///
/// You can find syntactic sugar to create `Command`s more easily in [macros](../index.html#macros)
///
/// # Examples
///
/// ```
/// extern crate ast;
///
/// let command: ast::structure::Command = ast::structure::Command::Skip;
/// ```
#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq,Copy)]
pub enum Command {
	Skip,
	Left,
	Right,
	Thrust
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Skip => write!(f, "Skip"),
            Command::Left => write!(f, "Left"),
            Command::Right => write!(f, "Right"),
            Command::Thrust => write!(f, "Thrust")
        }
    }
}

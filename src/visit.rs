//! Tree visitor routines

use super::structure::{Program,Condition,Command,Expression,Sensor};

pub trait Visitor<'a> {
	fn visit_program(&mut self, program: &'a Program);
	fn visit_condition(&mut self, condition: &'a Condition);
	fn visit_command(&mut self, command: &'a Command);
	fn visit_expression(&mut self, expression: &'a Expression);
	fn visit_sensor(&mut self, sensor: &'a Sensor);
}

pub trait Visitable<'a> {
    fn visit(&'a self, visitor: &mut Visitor<'a>);
}

impl <'a> Visitable<'a> for Program {
	fn visit(&'a self, visitor: &mut Visitor<'a>) {
        visitor.visit_program(self);

		match *self {
			Program::If(ref condition, ref left, ref right) => { condition.visit(visitor); left.visit(visitor); right.visit(visitor); },
			Program::Command(ref command) => { command.visit(visitor); }
		}
    }
}

impl <'a> Visitable<'a> for Condition {
	fn visit(&'a self, visitor: &mut Visitor<'a>) {
        visitor.visit_condition(self);

		match *self {
			Condition::Not(ref condition)                => { condition.visit(visitor); },
			Condition::Or(ref left, ref right)           => { left.visit(visitor); right.visit(visitor); },
			Condition::And(ref left, ref right)          => { left.visit(visitor); right.visit(visitor); },
			Condition::Less(ref left, ref right)         => { left.visit(visitor); right.visit(visitor); },
			Condition::LessEqual(ref left, ref right)    => { left.visit(visitor); right.visit(visitor); },
			Condition::Equal(ref left, ref right)        => { left.visit(visitor); right.visit(visitor); },
			Condition::GreaterEqual(ref left, ref right) => { left.visit(visitor); right.visit(visitor); },
			Condition::Greater(ref left, ref right)      => { left.visit(visitor); right.visit(visitor); },
            _ => {}
		}
	}
}

impl <'a> Visitable<'a> for Command {
	fn visit(&'a self, visitor: &mut Visitor<'a>) {
        visitor.visit_command(self);
    }
}

impl <'a> Visitable<'a> for Expression {
	fn visit(&'a self, visitor: &mut Visitor<'a>) {
        visitor.visit_expression(self);

		match *self {
			Expression::Sensor(ref sensor)            => { sensor.visit(visitor); },
			Expression::Plus(ref left, ref right)     => { left.visit(visitor); right.visit(visitor); },
			Expression::Minus(ref left, ref right)    => { left.visit(visitor); right.visit(visitor); },
			Expression::Multiply(ref left, ref right) => { left.visit(visitor); right.visit(visitor); },
			Expression::Divide(ref left, ref right)   => { left.visit(visitor); right.visit(visitor); },
            _ => {}
		}
    }
}

impl <'a> Visitable<'a> for Sensor {
	fn visit(&'a self, visitor: &mut Visitor<'a>) {
        visitor.visit_sensor(self);
    }
}

/// A simple visitor that collects references to all node types in a list of references
pub struct BucketCollector<'a> {
    programs: Vec<&'a Program>,
    conditions: Vec<&'a Condition>,
    commands: Vec<&'a Command>,
    expressions: Vec<&'a Expression>,
    sensors: Vec<&'a Sensor>,
}

impl <'a> BucketCollector<'a> {
    pub fn new() -> BucketCollector<'a> {
        BucketCollector {
            programs: Vec::new(),
            conditions: Vec::new(),
            commands: Vec::new(),
            expressions: Vec::new(),
            sensors: Vec::new()
        }
    }
}

impl <'a> Visitor<'a> for BucketCollector<'a> {
	fn visit_program(&mut self, program: &'a Program) {
        self.programs.push(program);
    }

	fn visit_condition(&mut self, condition: &'a Condition) {
        self.conditions.push(condition);
    }

	fn visit_command(&mut self, command: &'a Command) {
        self.commands.push(command);
    }

	fn visit_expression(&mut self, expression: &'a Expression) {
        self.expressions.push(expression);
    }

	fn visit_sensor(&mut self, sensor: &'a Sensor) {
        self.sensors.push(sensor);
    }
}

#[cfg(test)]
mod tests {
    // This makes the macros work (which expect stuff to be in ast::structure::...etc...)
    mod ast { pub use super::super::super::*; }

    use super::*;

    use super::super::structure::Program;

    #[test]
    fn collect_all_nodes() {
        let program: Program = iff!(less!(vx!(),multiply!(constant!(2.0000),constant!(3.0000))),left!(),right!());

        let mut coll = BucketCollector::new();
        program.visit(&mut coll);

        assert_eq!(3, coll.programs.len()); // iff, left, and right
        assert_eq!(1, coll.conditions.len()); // less
        assert_eq!(2, coll.commands.len());  // left, right
        assert_eq!(4, coll.expressions.len()); // vx, multiply, constant, constant
        assert_eq!(1, coll.sensors.len()); // vx
    }
}

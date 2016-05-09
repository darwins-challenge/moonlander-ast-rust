//! Tree rebuilding routines
//!
//! I'm calling it 'zipit' because I don't actually know whether this is a zipper or not :).

use super::structure::{Program,Condition,Command,Expression,Sensor};

pub trait Copier {
    fn copy_program(&self, program: &Program) -> Box<Program>;
    fn copy_condition(&self, condition: &Condition) -> Box<Condition>;
    fn copy_command(&self, command: &Command) -> Box<Command>;
    fn copy_expression(&self, expression: &Expression) -> Box<Expression>;
    fn copy_sensor(&self, sensor: &Sensor) -> Box<Sensor>;
}

pub trait Copyable {
    fn copy(&self, copier: &Copier) -> Box<Self>;
}

impl Copyable for Program {
    fn copy(&self, copier: &Copier) -> Box<Self> {
        copier.copy_program(self)
    }
}

impl Copyable for Condition {
    fn copy(&self, copier: &Copier) -> Box<Self> {
        copier.copy_condition(self)
    }
}

impl Copyable for Expression {
    fn copy(&self, copier: &Copier) -> Box<Self> {
        copier.copy_expression(self)
    }
}

impl Copyable for Command {
    fn copy(&self, copier: &Copier) -> Box<Self> {
        copier.copy_command(self)
    }
}

impl Copyable for Sensor {
    fn copy(&self, copier: &Copier) -> Box<Self> {
        copier.copy_sensor(self)
    }
}

pub struct CopyReplaceProgram<'a> {
    pub to_replace: &'a Program, pub replace_with: &'a Program
}

pub fn ref_eq<T>(a: &T, b: &T) -> bool {
    a as *const T == b as *const T
}

impl <'a> Copier for CopyReplaceProgram<'a> {
    fn copy_program(&self, program: &Program) -> Box<Program> { 
        copy_program(self, if ref_eq(program, self.to_replace) { self.replace_with } else { program })
    }
    fn copy_condition(&self, condition: &Condition) -> Box<Condition> { copy_condition(self, condition) }
    fn copy_command(&self, command: &Command) -> Box<Command> { copy_command(self, command) }
    fn copy_expression(&self, expression: &Expression) -> Box<Expression> { copy_expression(self, expression) }
    fn copy_sensor(&self, sensor: &Sensor) -> Box<Sensor> { copy_sensor(self, sensor) }
}

pub struct CopyReplaceCondition<'a> {
    pub to_replace: &'a Condition, pub replace_with: &'a Condition
}

impl <'a> Copier for CopyReplaceCondition<'a> {
    fn copy_program(&self, program: &Program) -> Box<Program> { copy_program(self, program) }
    fn copy_condition(&self, condition: &Condition) -> Box<Condition> {
        copy_condition(self, if ref_eq(condition, self.to_replace) { self.replace_with } else { condition })
    }
    fn copy_command(&self, command: &Command) -> Box<Command> { copy_command(self, command) }
    fn copy_expression(&self, expression: &Expression) -> Box<Expression> { copy_expression(self, expression) }
    fn copy_sensor(&self, sensor: &Sensor) -> Box<Sensor> { copy_sensor(self, sensor) }
}

pub struct CopyReplaceCommand<'a> {
    pub to_replace: &'a Command, pub replace_with: &'a Command
}

impl <'a> Copier for CopyReplaceCommand<'a> {
    fn copy_program(&self, program: &Program) -> Box<Program> { copy_program(self, program) }
    fn copy_condition(&self, condition: &Condition) -> Box<Condition> { copy_condition(self, condition) }
    fn copy_command(&self, command: &Command) -> Box<Command> {
        copy_command(self, if ref_eq(command, self.to_replace) { self.replace_with } else { command })
    }
    fn copy_expression(&self, expression: &Expression) -> Box<Expression> { copy_expression(self, expression) }
    fn copy_sensor(&self, sensor: &Sensor) -> Box<Sensor> { copy_sensor(self, sensor) }
}

pub struct CopyReplaceExpression<'a> {
    pub to_replace: &'a Expression, pub replace_with: &'a Expression
}

impl <'a> Copier for CopyReplaceExpression<'a> {
    fn copy_program(&self, program: &Program) -> Box<Program> { copy_program(self, program) }
    fn copy_condition(&self, condition: &Condition) -> Box<Condition> { copy_condition(self, condition) }
    fn copy_command(&self, command: &Command) -> Box<Command> { copy_command(self, command) }
    fn copy_expression(&self, expression: &Expression) -> Box<Expression> {
        copy_expression(self, if ref_eq(expression, self.to_replace) { self.replace_with } else { expression })
    }
    fn copy_sensor(&self, sensor: &Sensor) -> Box<Sensor> { copy_sensor(self, sensor) }
}

pub struct CopyReplaceSensor<'a> {
    pub to_replace: &'a Sensor, pub replace_with: &'a Sensor
}

impl <'a> Copier for CopyReplaceSensor<'a> {
    fn copy_program(&self, program: &Program) -> Box<Program> { copy_program(self, program) }
    fn copy_condition(&self, condition: &Condition) -> Box<Condition> { copy_condition(self, condition) }
    fn copy_command(&self, command: &Command) -> Box<Command> { copy_command(self, command) }
    fn copy_expression(&self, expression: &Expression) -> Box<Expression> { copy_expression(self, expression) }
    fn copy_sensor(&self, sensor: &Sensor) -> Box<Sensor> {
        copy_sensor(self, if ref_eq(sensor, self.to_replace) { self.replace_with } else { sensor })
    }
}

fn copy_program(copier: &Copier, program: &Program) -> Box<Program> {
    Box::new(match *program {
        Program::If(ref condition, ref left, ref right) => Program::If(condition.copy(copier), left.copy(copier), right.copy(copier)),
        Program::Command(ref command) => Program::Command(command.copy(copier))
    })
}

fn copy_expression(copier: &Copier, expression: &Expression) -> Box<Expression> {
    Box::new(match *expression {
        Expression::Sensor(ref sensor)            => Expression::Sensor(sensor.copy(copier)),
        Expression::Plus(ref left, ref right)     => Expression::Plus(left.copy(copier), right.copy(copier)),
        Expression::Minus(ref left, ref right)    => Expression::Minus(left.copy(copier), right.copy(copier)),
        Expression::Multiply(ref left, ref right) => Expression::Multiply(left.copy(copier), right.copy(copier)),
        Expression::Divide(ref left, ref right)   => Expression::Divide(left.copy(copier), right.copy(copier)),
        _ => expression.clone()
    })
}

fn copy_condition(copier: &Copier, condition: &Condition) -> Box<Condition> {
    Box::new(match *condition {
        Condition::Not(ref condition)                => Condition::Not(condition.copy(copier)),
        Condition::Or(ref left, ref right)           => Condition::Or(left.copy(copier), right.copy(copier)),
        Condition::And(ref left, ref right)          => Condition::And(left.copy(copier), right.copy(copier)),
        Condition::Less(ref left, ref right)         => Condition::Less(left.copy(copier), right.copy(copier)),
        Condition::LessEqual(ref left, ref right)    => Condition::LessEqual(left.copy(copier), right.copy(copier)),
        Condition::Equal(ref left, ref right)        => Condition::Equal(left.copy(copier), right.copy(copier)),
        Condition::GreaterEqual(ref left, ref right) => Condition::GreaterEqual(left.copy(copier), right.copy(copier)),
        Condition::Greater(ref left, ref right)      => Condition::Greater(left.copy(copier), right.copy(copier)),
        _                                            => condition.clone()
    })
}

fn copy_sensor(_: &Copier, sensor: &Sensor) -> Box<Sensor> {
    Box::new(sensor.clone())
}

fn copy_command(_: &Copier, command: &Command) -> Box<Command> {
    Box::new(command.clone())
}

#[cfg(test)]
mod tests {
    // This makes the macros work (which expect stuff to be in ast::structure::...etc...)
    mod ast { pub use super::super::super::*; }

    use super::super::visit::{Visitable, BucketCollector};
    use super::*;

    #[test]
    fn copy_and_replace() {
        let program = iff!(less_equal!(constant!(0.9750),minus!(minus!(multiply!(minus!(y!(),minus!(divide!(constant!(0.9907),y!()),w!())),vy!()),o!()),constant!(0.7831))),skip!(),thrust!());
        let replacement = iff!(less_equal!(constant!(0.9750),minus!(minus!(multiply!(minus!(y!(),minus!(divide!(constant!(0.9907),y!()),w!())),vy!()),o!()),constant!(0.7831))),skip!(),thrust!());

        let mut nodes = BucketCollector::new();
        program.visit(&mut nodes);

        let replacer = CopyReplaceProgram { to_replace: nodes.programs[1], replace_with: &replacement };
        let copied = program.copy(&replacer);

        println!("{:?}", copied);
            
        // Count the Programs in the new tree, it'll be 5
        let mut new_nodes = BucketCollector::new();
        copied.visit(&mut new_nodes);
        assert_eq!(5, new_nodes.programs.len());
    }
}

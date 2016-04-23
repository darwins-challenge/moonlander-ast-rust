/// The `mutate` trait returns a mutation version of a type that implements it.
///
/// Implementer should try to create persisted data-structures

extern crate rand;

use self::rand::Rng;
use super::super::structure::{Program,Condition,Expression,Sensor,Command};

pub trait Mutate {
    fn mutate(&self) -> Self;
}

enum MutateType {
    ThisLevel,
    NextLevel,
}

impl rand::Rand for MutateType {
    fn rand<R: rand::Rng>(_: &mut R) -> Self {
        pick![
            1, MutateType::ThisLevel,
            1, MutateType::NextLevel
        ]
    }
}

impl Mutate for Program {
    fn mutate(&self) -> Program {
        let mutate_type: MutateType = rand::random();
        match mutate_type {
            MutateType::ThisLevel => {
                let mutation: Program = rand::random();
                mutation
            },
            MutateType::NextLevel => {
                match *self {
                    Program::If(ref condition, ref left, ref right) => {
                        let mut rng = rand::thread_rng();
                        match rng.gen_range(0, 3) {
                            0 => { /* mutate condition */
                                      let mutation: Condition = condition.mutate();
                                      Program::If(mutation, left.clone(), right.clone())
                            },
                            1 => { /* mutate left */
                                      let mutation: Program = left.mutate();
                                      Program::If(condition.clone(), Box::new(mutation), right.clone())
                            },
                            2 => { /* mutate right */
                                      let mutation: Program = right.mutate();
                                      Program::If(condition.clone(), left.clone(), Box::new(mutation))
                            },
                            _ => { panic!(/* this should not happen */)},
                        }
                    },
                    Program::Command(ref command) => {
                        let mutation: Command = command.mutate();
                        Program::Command(mutation)
                    }
                } 
            },
        }
    }
}

impl Mutate for Condition {
    fn mutate(&self) -> Condition {
        let mutate_type: MutateType = rand::random();
        match mutate_type {
            MutateType::ThisLevel => {
                let mutation: Condition = rand::random();
                mutation
            },
            MutateType::NextLevel => {
                match *self {
                    Condition::True => Condition::False,
                    Condition::False => Condition::True,
                    Condition::Not(ref condition) => {
                        let mutation: Condition = condition.mutate();
                        Condition::Not(Box::new(mutation))
                    },
                    Condition::Or(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Condition = left.mutate();
                            Condition::Or(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Condition = right.mutate();
                            Condition::Or(left.clone(), Box::new(mutation))
                        }
                    },
                    Condition::And(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Condition = left.mutate();
                            Condition::And(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Condition = right.mutate();
                            Condition::And(left.clone(), Box::new(mutation))
                        }
                    },
                    Condition::Less(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate();
                            Condition::Less(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate();
                            Condition::Less(left.clone(), Box::new(mutation))
                        }
                    },
                    Condition::LessEqual(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate();
                            Condition::LessEqual(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate();
                            Condition::LessEqual(left.clone(), Box::new(mutation))
                        }
                    },
                    Condition::Equal(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate();
                            Condition::Equal(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate();
                            Condition::Equal(left.clone(), Box::new(mutation))
                        }
                    },
                    Condition::GreaterEqual(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate();
                            Condition::GreaterEqual(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate();
                            Condition::GreaterEqual(left.clone(), Box::new(mutation))
                        }
                    },
                    Condition::Greater(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate();
                            Condition::Greater(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate();
                            Condition::Greater(left.clone(), Box::new(mutation))
                        }
                    }
                }
            }
        }
    }
}

impl Mutate for Expression {
    fn mutate(&self) -> Expression {
        let mutate_type: MutateType = rand::random();
        match mutate_type {
            MutateType::ThisLevel => {
                let mutation: Expression = rand::random();
                mutation
            },
            MutateType::NextLevel => {
                match *self {
                    Expression::Constant(_) => {
                        let mutation: f32 = rand::random();
                        Expression::Constant(mutation)
                    },
                    Expression::Sensor(ref sensor) => {
                        let mutation: Sensor = sensor.mutate();
                        Expression::Sensor(mutation)
                    },
                    Expression::Plus(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate();
                            Expression::Plus(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate();
                            Expression::Plus(left.clone(),Box::new(mutation))
                        }
                    },
                    Expression::Minus(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate();
                            Expression::Minus(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate();
                            Expression::Minus(left.clone(), Box::new(mutation))
                        }
                    },
                    Expression::Multiply(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate();
                            Expression::Multiply(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate();
                            Expression::Multiply(left.clone(), Box::new(mutation))
                        }
                    },
                    Expression::Divide(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate();
                            Expression::Divide(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate();
                            Expression::Divide(left.clone(), Box::new(mutation))
                        }
                    },
                }
            }
        }
    }
}

impl Mutate for Sensor {
    fn mutate(&self) -> Sensor {
        let mutation: Sensor = rand::random();
        mutation
    }
}

impl Mutate for Command {
    fn mutate(&self) -> Command {
        let mutation: Command = rand::random();
        mutation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use structure::{Expression,Sensor};

    #[derive(PartialEq,Debug)]
    enum Dummy {
        A, B,
    }

    impl super::Mutate for Dummy {
        fn mutate(&self) -> Dummy {
            match *self {
                Dummy::A => Dummy::B,
                Dummy::B => Dummy::A
            }
        }
    }

    #[test]
    fn should_mutate_mutate_dummy() {
        let original = Dummy::A;

        let mutation = original.mutate();

        assert_eq!(mutation, Dummy::B);
    }

    #[test]
    fn should_mutate_expression_without_panicing() {
        let constant: Expression = Expression::Constant(0.0);
        constant.mutate();

        let sensor: Expression = Expression::Sensor(Sensor::Vx);
        sensor.mutate();
    }
}

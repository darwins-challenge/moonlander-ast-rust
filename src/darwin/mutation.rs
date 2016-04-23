/// The `mutate` trait returns a mutation version of a type that implements it.
///
/// Implementer should try to create persisted data-structures

extern crate rand;

use self::rand::Rng;
use super::super::structure::{Program,Condition,Expression,Sensor,Command};

pub trait Mutate {
    fn mutate<R: rand::Rng>(&self, rng: &mut R) -> Self;
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
    fn mutate<R: rand::Rng>(&self, rng: &mut R) -> Program {
        let mutate_type: MutateType = rand::random();
        match mutate_type {
            MutateType::ThisLevel => {
                let mutation: Program = rand::random();
                mutation
            },
            MutateType::NextLevel => {
                match *self {
                    Program::If(ref condition, ref left, ref right) => {
                        match rng.gen_range(0, 3) {
                            0 => { /* mutate condition */
                                      let mutation: Condition = condition.mutate(rng);
                                      Program::If(mutation, left.clone(), right.clone())
                            },
                            1 => { /* mutate left */
                                      let mutation: Program = left.mutate(rng);
                                      Program::If(condition.clone(), Box::new(mutation), right.clone())
                            },
                            2 => { /* mutate right */
                                      let mutation: Program = right.mutate(rng);
                                      Program::If(condition.clone(), left.clone(), Box::new(mutation))
                            },
                            _ => { panic!(/* this should not happen */)},
                        }
                    },
                    Program::Command(ref command) => {
                        let mutation: Command = command.mutate(rng);
                        Program::Command(mutation)
                    }
                } 
            },
        }
    }
}

impl Mutate for Condition {
    fn mutate<R: rand::Rng>(&self, rng: &mut R) -> Condition {
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
                        let mutation: Condition = condition.mutate(rng);
                        Condition::Not(Box::new(mutation))
                    },
                    Condition::Or(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Condition = left.mutate(rng);
                            Condition::Or(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Condition = right.mutate(rng);
                            Condition::Or(left.clone(), Box::new(mutation))
                        }
                    },
                    Condition::And(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Condition = left.mutate(rng);
                            Condition::And(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Condition = right.mutate(rng);
                            Condition::And(left.clone(), Box::new(mutation))
                        }
                    },
                    Condition::Less(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate(rng);
                            Condition::Less(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate(rng);
                            Condition::Less(left.clone(), Box::new(mutation))
                        }
                    },
                    Condition::LessEqual(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate(rng);
                            Condition::LessEqual(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate(rng);
                            Condition::LessEqual(left.clone(), Box::new(mutation))
                        }
                    },
                    Condition::Equal(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate(rng);
                            Condition::Equal(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate(rng);
                            Condition::Equal(left.clone(), Box::new(mutation))
                        }
                    },
                    Condition::GreaterEqual(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate(rng);
                            Condition::GreaterEqual(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate(rng);
                            Condition::GreaterEqual(left.clone(), Box::new(mutation))
                        }
                    },
                    Condition::Greater(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate(rng);
                            Condition::Greater(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate(rng);
                            Condition::Greater(left.clone(), Box::new(mutation))
                        }
                    }
                }
            }
        }
    }
}

impl Mutate for Expression {
    fn mutate<R: rand::Rng>(&self, rng: &mut R) -> Expression {
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
                        let mutation: Sensor = sensor.mutate(rng);
                        Expression::Sensor(mutation)
                    },
                    Expression::Plus(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate(rng);
                            Expression::Plus(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate(rng);
                            Expression::Plus(left.clone(),Box::new(mutation))
                        }
                    },
                    Expression::Minus(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate(rng);
                            Expression::Minus(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate(rng);
                            Expression::Minus(left.clone(), Box::new(mutation))
                        }
                    },
                    Expression::Multiply(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate(rng);
                            Expression::Multiply(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate(rng);
                            Expression::Multiply(left.clone(), Box::new(mutation))
                        }
                    },
                    Expression::Divide(ref left, ref right) => {
                        let mutate_left: bool = rand::random();
                        if mutate_left {
                            let mutation: Expression = left.mutate(rng);
                            Expression::Divide(Box::new(mutation), right.clone())
                        } else {
                            let mutation: Expression = right.mutate(rng);
                            Expression::Divide(left.clone(), Box::new(mutation))
                        }
                    },
                }
            }
        }
    }
}

impl Mutate for Sensor {
    fn mutate<R: rand::Rng>(&self, rng: &mut R) -> Sensor {
        let mutation: Sensor = rand::random();
        mutation
    }
}

impl Mutate for Command {
    fn mutate<R: rand::Rng>(&self, rng: &mut R) -> Command {
        let mutation: Command = rand::random();
        mutation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::rand::Rng;
    use structure::{Expression,Sensor};

    #[derive(PartialEq,Debug)]
    enum Dummy {
        A, B,
    }

    impl super::Mutate for Dummy {
        fn mutate<R: super::rand::Rng>(&self, rng: &mut R) -> Dummy {
            match *self {
                Dummy::A => Dummy::B,
                Dummy::B => Dummy::A
            }
        }
    }

    #[test]
    fn should_mutate_mutate_dummy() {
        let original = Dummy::A;
        let mut rng = super::rand::thread_rng();

        let mutation = original.mutate(&mut rng);

        assert_eq!(mutation, Dummy::B);
    }

    #[test]
    fn should_mutate_expression_without_panicing() {
        let mut rng = super::rand::thread_rng();
        let constant: Expression = Expression::Constant(0.0);
        constant.mutate(&mut rng);

        let sensor: Expression = Expression::Sensor(Sensor::Vx);
        sensor.mutate(&mut rng);
    }
}

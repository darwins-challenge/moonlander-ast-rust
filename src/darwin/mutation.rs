/// The `mutate` trait returns a mutation version of a type that implements it.
///
/// Implementer should try to create persisted data-structures

extern crate rand;

use super::super::structure::{Expression,Sensor,Command};

pub trait Mutate {
    fn mutate(&self) -> Self;
}

enum MutateType {
    ThisLevel,
    NextLevel,
}

impl Mutate for Expression {
    fn mutate(&self) -> Expression {
        match *self {
            Expression::Constant(_) => {
                let mutation: f32 = rand::random();
                Expression::Constant(mutation)
            },
            _ => panic!(),
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
    use structure::{Expression};

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
        
    }
}

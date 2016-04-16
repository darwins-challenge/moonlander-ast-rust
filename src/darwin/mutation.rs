/// The `mutate` trait returns a mutation version of a type that implements it.
///
/// Implementer are should try to create persisted data-structures

extern crate rand;

use super::super::structure::{Sensor,Command};

pub trait Mutate {
    fn mutate(&self) -> Self;
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
}

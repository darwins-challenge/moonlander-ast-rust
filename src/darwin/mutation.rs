/// The `mutate` trait returns a mutation version of a type that implements it.
///
/// Implementer are should try to create persisted data-structures

pub trait Mutate {
    fn mutate(&self) -> Self;
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

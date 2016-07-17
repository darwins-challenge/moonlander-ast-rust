/// The `mutate` trait returns a mutation version of a type that implements it.
///
/// Implementer should try to create persisted data-structures

extern crate rand;

use self::rand::Rng;
use self::rand::Rand;
use super::super::structure::{Program, Sensor, Command, Expression, Condition, Number};
use super::super::visit::{NodeType, Visitable, BucketCollector};
use super::super::copy;
use super::super::copy::Copyable;

pub fn mutate<'a, T, R>(a: &'a T, rng: &mut R) -> T where 
    T: Visitable<'a>+Copyable,
    R: rand::Rng+Sized
{
    let mut nodes = BucketCollector::new();
    a.visit(&mut nodes);

    let t_nodes = nodes.get_counts().into_iter()
        .filter(|&(_, nc)| nc > 0)
        .map(|(t, _)| t)
        .collect::<Vec<NodeType>>();

    let picked_type = rng.choose(&t_nodes).unwrap();

    match *picked_type {
        NodeType::Program    => mut_program(a, rng.choose(&nodes.programs).unwrap(), rng),
        NodeType::Expression => mut_expression(a, rng.choose(&nodes.expressions).unwrap(), rng),
        NodeType::Condition  => mut_condition(a, rng.choose(&nodes.conditions).unwrap(), rng),
        NodeType::Sensor     => mut_sensor(a, rng.choose(&nodes.sensors).unwrap(), rng),
        NodeType::Command    => mut_command(a, rng.choose(&nodes.commands).unwrap(), rng)
    }
}

fn mut_program<P: Copyable, R: rand::Rng>(a: &P, an: &Program, rng: &mut R) -> P {
    let bn = Program::rand(rng);
    // FIXME: Also do controlled_mutate_program
    a.copy(&copy::CopyReplaceProgram { to_replace: an, replace_with: &bn })
}

fn mut_expression<P: Copyable, R: rand::Rng>(a: &P, an: &Expression, rng: &mut R) -> P {
    let bn = pick![
        1, Expression::rand(rng),
        4, controlled_mutate_expression(an, rng)
        ];
    a.copy(&copy::CopyReplaceExpression { to_replace: an, replace_with: &bn })
}

fn mut_condition<P: Copyable, R: rand::Rng>(a: &P, an: &Condition, rng: &mut R) -> P {
    let bn = pick![
        1, Condition::rand(rng),
        4, controlled_mutate_condition(an, rng)
        ];
    a.copy(&copy::CopyReplaceCondition { to_replace: an, replace_with: &bn })
}

fn mut_sensor<P: Copyable, R: rand::Rng>(a: &P, an: &Sensor, rng: &mut R) -> P {
    let bn = Sensor::rand(rng);
    a.copy(&copy::CopyReplaceSensor { to_replace: an, replace_with: &bn })
}

fn mut_command<P: Copyable, R: rand::Rng>(a: &P, an: &Command, rng: &mut R) -> P {
    let bn = Command::rand(rng);
    a.copy(&copy::CopyReplaceCommand { to_replace: an, replace_with: &bn })
}

fn mutate_constant<R: rand::Rng>(value: Number, rng: &mut R) -> Number {
    pick![
        1, Number::rand(rng),                 // Completely new random number
        7, value * rng.gen_range(0.1, 2.0),   // Twiddled random number
        2, value + rng.gen_range(-1.0, 1.0)   // Move away from 0
    ]
}

fn controlled_mutate_condition<R: rand::Rng>(c: &Condition, rng: &mut R) -> Condition {
    match *c {
        Condition::True                       => condition_with_conditions(&[], rng),
        Condition::False                      => condition_with_conditions(&[], rng),
        Condition::Not(ref l)                 => condition_with_conditions(&[l], rng),
        Condition::Or(ref l, ref r)           => condition_with_conditions(&[l, r], rng),
        Condition::And(ref l, ref r)          => condition_with_conditions(&[l, r], rng),

        Condition::Less(ref l, ref r)         => condition_with_expressions(&[l, r], rng),
        Condition::LessEqual(ref l, ref r)    => condition_with_expressions(&[l, r], rng),
        Condition::Equal(ref l, ref r)        => condition_with_expressions(&[l, r], rng),
        Condition::GreaterEqual(ref l, ref r) => condition_with_expressions(&[l, r], rng),
        Condition::Greater(ref l, ref r)      => condition_with_expressions(&[l, r], rng),
    }
}

fn controlled_mutate_expression<R: rand::Rng>(e: &Expression, rng: &mut R) -> Expression {
    match *e {
        Expression::Constant(c)              => Expression::Constant(mutate_constant(c, rng)),
        Expression::Sensor(_)                => Expression::Sensor(Box::new(Sensor::rand(rng))),
        Expression::Plus(ref l, ref r)       => expression_with_expressions(&[l, r], rng),
        Expression::Minus(ref l, ref r)      => expression_with_expressions(&[l, r], rng),
        Expression::Multiply(ref l, ref r)   => expression_with_expressions(&[l, r], rng),
        Expression::Divide(ref l, ref r)     => expression_with_expressions(&[l, r], rng)
    }
}

fn condition_with_conditions<R: rand::Rng>(cs: &[&Condition], rng: &mut R) -> Condition
{
    let i = pick![8, 0, 2, 1];  // Mostly the same, some chance to reverse
    let j = 1 - i;

    pick![
        1, Condition::True,
        1, Condition::False,
        1, Condition::Not(Box::new(pick_or_rand(cs, i, rng))),
        1, Condition::Or(Box::new(pick_or_rand(cs, i, rng)), Box::new(pick_or_rand(cs, j, rng))),
        1, Condition::And(Box::new(pick_or_rand(cs, i, rng)), Box::new(pick_or_rand(cs, j, rng)))
        ]
}

fn condition_with_expressions<R: rand::Rng>(es: &[&Expression], rng: &mut R) -> Condition
{
    let i = pick![8, 0, 2, 1];  // Mostly the same, some chance to reverse the arms
    let j = 1 - i;

    pick![
        1, Condition::Less(Box::new(pick_or_rand(es, i, rng)), Box::new(pick_or_rand(es, j, rng))),
        1, Condition::LessEqual(Box::new(pick_or_rand(es, i, rng)), Box::new(pick_or_rand(es, j, rng))),
        1, Condition::Equal(Box::new(pick_or_rand(es, i, rng)), Box::new(pick_or_rand(es, j, rng))),
        1, Condition::GreaterEqual(Box::new(pick_or_rand(es, i, rng)), Box::new(pick_or_rand(es, j, rng))),
        1, Condition::Greater(Box::new(pick_or_rand(es, i, rng)), Box::new(pick_or_rand(es, j, rng)))
        ]
}

fn expression_with_expressions<R: rand::Rng>(es: &[&Expression], rng: &mut R) -> Expression
{
    let i = pick![8, 0, 2, 1];  // Mostly the same, some chance to reverse the arms
    let j = 1 - i;

    pick![
        1, Expression::Plus(Box::new(pick_or_rand(es, i, rng)), Box::new(pick_or_rand(es, j, rng))),
        1, Expression::Minus(Box::new(pick_or_rand(es, i, rng)), Box::new(pick_or_rand(es, j, rng))),
        1, Expression::Multiply(Box::new(pick_or_rand(es, i, rng)), Box::new(pick_or_rand(es, j, rng))),
        1, Expression::Divide(Box::new(pick_or_rand(es, i, rng)), Box::new(pick_or_rand(es, j, rng)))
        ]
}

/// Pick from an array or generate a random new element
fn pick_or_rand<T: rand::Rand+Clone, R: rand::Rng>(cs: &[&T], i: usize, rng: &mut R) -> T {
    if i < cs.len() {
        (*cs[i]).clone()
    } else {
        T::rand(rng)
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
        fn mutate<R: super::rand::Rng>(&self, _: &mut R) -> Dummy {
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

        let sensor: Expression = Expression::Sensor(Box::new(Sensor::Vx));
        sensor.mutate(&mut rng);
    }
}

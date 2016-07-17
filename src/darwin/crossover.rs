//! 

use rand;

use super::super::visit::{NodeType, Visitable, BucketCollector};
use super::super::structure::{Program, Sensor, Command, Expression, Condition};
use super::super::copy;
use super::super::copy::Copyable;

pub fn cross_over<'a, P, H>(a: &'a P, b: &'a P, rng: &mut H) -> (P, P) where 
    P: Visitable<'a>+Copyable,
    H: rand::Rng+Sized
{
    let mut a_nodes = BucketCollector::new();
    let mut b_nodes = BucketCollector::new();

    debug_assert!(!copy::ref_eq(a, b));

    a.visit(&mut a_nodes);
    b.visit(&mut b_nodes);

    let nodes = a_nodes.get_counts().into_iter()
        .zip(b_nodes.get_counts().into_iter())
        .filter(|&((_, ac), (_, bc))| ac > 0 && bc > 0)
        .map(|((t, _), (_, _))| t)
        .collect::<Vec<NodeType>>();

    let picked_type = rng.choose(&nodes).unwrap();

    match *picked_type {
        NodeType::Program    => cross_over_program(a, b, rng.choose(&a_nodes.programs).unwrap(), rng.choose(&b_nodes.programs).unwrap()),
        NodeType::Expression => cross_over_expression(a, b, rng.choose(&a_nodes.expressions).unwrap(), rng.choose(&b_nodes.expressions).unwrap()),
        NodeType::Condition  => cross_over_condition(a, b, rng.choose(&a_nodes.conditions).unwrap(), rng.choose(&b_nodes.conditions).unwrap()),
        NodeType::Sensor     => cross_over_sensor(a, b, rng.choose(&a_nodes.sensors).unwrap(), rng.choose(&b_nodes.sensors).unwrap()),
        NodeType::Command    => cross_over_command(a, b, rng.choose(&a_nodes.commands).unwrap(), rng.choose(&b_nodes.commands).unwrap())
    }
}

fn cross_over_program<P: Copyable>(a: &P, b: &P, an: &Program, bn: &Program) -> (P, P) {
    (a.copy(&copy::CopyReplaceProgram { to_replace: an, replace_with: bn }),
     b.copy(&copy::CopyReplaceProgram { to_replace: bn, replace_with: an }))
}

fn cross_over_expression<P: Copyable>(a: &P, b: &P, an: &Expression, bn: &Expression) -> (P, P) {
    (a.copy(&copy::CopyReplaceExpression { to_replace: an, replace_with: bn }),
     b.copy(&copy::CopyReplaceExpression { to_replace: bn, replace_with: an }))
}

fn cross_over_condition<P: Copyable>(a: &P, b: &P, an: &Condition, bn: &Condition) -> (P, P) {
    (a.copy(&copy::CopyReplaceCondition { to_replace: an, replace_with: bn }),
     b.copy(&copy::CopyReplaceCondition { to_replace: bn, replace_with: an }))
}

fn cross_over_sensor<P: Copyable>(a: &P, b: &P, an: &Sensor, bn: &Sensor) -> (P, P) {
    (a.copy(&copy::CopyReplaceSensor { to_replace: an, replace_with: bn }),
     b.copy(&copy::CopyReplaceSensor { to_replace: bn, replace_with: an }))
}

fn cross_over_command<P: Copyable>(a: &P, b: &P, an: &Command, bn: &Command) -> (P, P) {
    (a.copy(&copy::CopyReplaceCommand { to_replace: an, replace_with: bn }),
     b.copy(&copy::CopyReplaceCommand { to_replace: bn, replace_with: an }))
}

#[cfg(test)]
mod tests {
    // This makes the macros work (which expect stuff to be in ast::structure::...etc...)
    mod ast { pub use super::super::super::*; }

    use super::super::super::structure::Program;
    use super::*;
    use rand;
    use rand::Rng;

    #[test]
    fn crossover_random_trees() {
        // Cross over some random trees to see we don't stackoverflow
        let mut program1 = rand::thread_rng().gen::<Program>();
        for _ in 0..100 {
            let program2 = rand::thread_rng().gen::<Program>();
            let (a, _) = cross_over(&program1, &program2, &mut rand::thread_rng());
            program1 = a;
        }
    }
}

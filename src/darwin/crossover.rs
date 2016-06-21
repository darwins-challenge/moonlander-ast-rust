//! 

use rand;

use super::super::visit::{Visitable, BucketCollector};
use super::super::structure::{Program, Sensor, Command, Expression, Condition};
use super::super::copy;
use super::super::copy::Copyable;

#[derive(Copy, Clone)]
enum NodeType {
    Program,
    Expression,
    Condition,
    Sensor,
    Command
}

pub fn cross_over<'a, P, H>(a: &'a P, b: &'a P, rng: &mut H) -> (P, P) where 
    P: Visitable<'a>+Copyable,
    H: rand::Rng+Sized
{
    let mut a_nodes = BucketCollector::new();
    let mut b_nodes = BucketCollector::new();

    debug_assert!(!copy::ref_eq(a, b));

    a.visit(&mut a_nodes);
    b.visit(&mut b_nodes);

    let nodes = get_counts(&a_nodes).into_iter()
        .zip(get_counts(&b_nodes).into_iter())
        .filter(|&((_, ac), (_, bc))| ac > 0 && bc > 0)
        .map(|((t, _), (_, _))| t)
        .collect::<Vec<NodeType>>();

    let picked_type = choose(nodes, rng);

    match picked_type {
        NodeType::Program    => cross_over_program(a, b, choose(a_nodes.programs, rng), choose(b_nodes.programs, rng)),
        NodeType::Expression => cross_over_expression(a, b, choose(a_nodes.expressions, rng), choose(b_nodes.expressions, rng)),
        NodeType::Condition  => cross_over_condition(a, b, choose(a_nodes.conditions, rng), choose(b_nodes.conditions, rng)),
        NodeType::Sensor     => cross_over_sensor(a, b, choose(a_nodes.sensors, rng), choose(b_nodes.sensors, rng)),
        NodeType::Command    => cross_over_command(a, b, choose(a_nodes.commands, rng), choose(b_nodes.commands, rng))
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

fn get_counts(coll: &BucketCollector) -> Vec<(NodeType, usize)> {
    vec![(NodeType::Program,    coll.programs.len()),
         (NodeType::Expression, coll.expressions.len()),
         (NodeType::Condition,  coll.conditions.len()),
         (NodeType::Sensor,     coll.sensors.len()),
         (NodeType::Command,    coll.commands.len())]
}

fn choose<T: Copy, H: rand::Rng + Sized>(vec: Vec<T>, rng: &mut H) -> T {
    let i = rng.gen_range(0, vec.len());
    vec[i]
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

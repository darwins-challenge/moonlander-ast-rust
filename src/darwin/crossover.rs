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

pub fn cross_over<H: rand::Rng + Sized>(a: &Program, b: &Program, rng: &mut H) -> (Box<Program>, Box<Program>) {
    let mut a_nodes = BucketCollector::new();
    let mut b_nodes = BucketCollector::new();

    a.visit(&mut a_nodes);
    b.visit(&mut b_nodes);

    let nodes = get_counts(&a_nodes).into_iter()
        .zip(get_counts(&b_nodes).into_iter())
        .filter(|&((_, ac), (_, bc))| ac > 0 && bc > 0)
        .map(|((t, _), (_, _))| t)
        .collect::<Vec<NodeType>>();

    let picked_type = choose(nodes, rng);

    match picked_type {
        NodeType::Program    => cross_over_program(a, b, choose(a_nodes.programs, rng), choose(b_nodes.programs, rng), rng),
        NodeType::Expression => cross_over_expression(a, b, choose(a_nodes.expressions, rng), choose(b_nodes.expressions, rng), rng),
        NodeType::Condition  => cross_over_condition(a, b, choose(a_nodes.conditions, rng), choose(b_nodes.conditions, rng), rng),
        NodeType::Sensor     => cross_over_sensor(a, b, choose(a_nodes.sensors, rng), choose(b_nodes.sensors, rng), rng),
        NodeType::Command    => cross_over_command(a, b, choose(a_nodes.commands, rng), choose(b_nodes.commands, rng), rng)
    }
}

fn cross_over_program<'a, H: rand::Rng+Sized>(a: &Program, b: &Program, an: &Program, bn: &Program, rng: &mut H) -> (Box<Program>, Box<Program>) {
    (a.copy(&copy::CopyReplaceProgram { to_replace: an, replace_with: bn }),
     b.copy(&copy::CopyReplaceProgram { to_replace: bn, replace_with: an }))
}

fn cross_over_expression<'a, H: rand::Rng+Sized>(a: &Program, b: &Program, an: &Expression, bn: &Expression, rng: &mut H) -> (Box<Program>, Box<Program>) {
    (a.copy(&copy::CopyReplaceExpression { to_replace: an, replace_with: bn }),
     b.copy(&copy::CopyReplaceExpression { to_replace: bn, replace_with: an }))
}

fn cross_over_condition<'a, H: rand::Rng+Sized>(a: &Program, b: &Program, an: &Condition, bn: &Condition, rng: &mut H) -> (Box<Program>, Box<Program>) {
    (a.copy(&copy::CopyReplaceCondition { to_replace: an, replace_with: bn }),
     b.copy(&copy::CopyReplaceCondition { to_replace: bn, replace_with: an }))
}

fn cross_over_sensor<'a, H: rand::Rng+Sized>(a: &Program, b: &Program, an: &Sensor, bn: &Sensor, rng: &mut H) -> (Box<Program>, Box<Program>) {
    (a.copy(&copy::CopyReplaceSensor { to_replace: an, replace_with: bn }),
     b.copy(&copy::CopyReplaceSensor { to_replace: bn, replace_with: an }))
}

fn cross_over_command<'a, H: rand::Rng+Sized>(a: &Program, b: &Program, an: &Command, bn: &Command, rng: &mut H) -> (Box<Program>, Box<Program>) {
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

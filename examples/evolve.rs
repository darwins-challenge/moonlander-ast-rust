//! Simple evolution example
#[macro_use]
extern crate ast;
extern crate rand;
extern crate rustc_serialize;

pub use rand::Rng;

use std::ops::Mul;
use ast::structure::Program;
use ast::simulation;
use ast::serialize;
use ast::simplify::Simplify;
use ast::data::SensorData;
use ast::darwin::evolve;
use std::f32::consts::PI;
use std::path::Path;

const POPULATION_SIZE : usize = 2000;
const TRIALS_PER_PROGRAM : u32 = 10;
const TOURNAMENT_SIZE : usize = 20;
const SAVE_EVERY : u32 = 100;

fn random_start_position<R: rand::Rng>(rng: &mut R) -> SensorData {
    SensorData::new()
        .with_x(0.0)
        .with_y(rng.next_f32() * 100.0 + 50.0)
        .with_o(rng.next_f32() * PI * 2.0)
        .build()
}

fn square<T: Mul+Copy>(x: T) -> T::Output {
    x * x
}

fn angle_dist(o: f32) -> f32 {
    let r = o % 2. * PI;
    if r > PI { 2. * PI - r } else { r }
}

/// Score a program by scoring a single run
///
/// Ultimate score is composed of:
/// - How many frames we survived (higher is better)
/// - What our maximum height was (lower is better)
/// - If we landed (if so then FUCK YEAH)
fn score_single_run(program: &Program) -> f64 {
    let mut rng = rand::thread_rng();
    let mut sensor_data = random_start_position(&mut rng);
    let world = simulation::World::new().build();

    let mut frames: u32 = 0;
    let mut total_height: f32 = 0.;
    let mut total_fuel: f32 = 0.;
    let mut total_angle: f32 = 0.;

    while !sensor_data.crashed && !sensor_data.landed {
        total_height += square(sensor_data.y);
        total_fuel += square(sensor_data.fuel);
        total_angle += square(angle_dist(sensor_data.o));

        simulation::next(&mut sensor_data, &program, &world);

        frames += 1;
    };

    let score =
        3.0 * frames as f32
        + -(0.001 * total_height / frames as f32) // Average height is penalty
        + -(3. * total_angle / frames as f32) // Penalty for angle distance
        + (100.0 * total_fuel / frames as f32) // Average fuel is too
        + if sensor_data.landed { 500.0 } else { 0.0 } 
        ;
    score as f64
}

/// Score a program by averaging the score of multiple random runs
fn score_program(program: &Program) -> f64 {
    let mut total = 0.0;
    for _ in 0..TRIALS_PER_PROGRAM {
        let score = score_single_run(program);
        if score > total { 
            total = score;
        }
    };
    total
}

fn save_trace(generation: u32, program: &Program) {
    let mut sensor_data = random_start_position(&mut rand::thread_rng());
    let world = simulation::World::new().build();
    let mut trace = serialize::GameTrace::new();

    trace.add(&sensor_data);
    while !sensor_data.crashed && !sensor_data.landed {
        simulation::next(&mut sensor_data, &program, &world);
        trace.add(&sensor_data);
    }

    let tracefile = format!("trace_{}.json", generation);
    trace.save(Path::new(&tracefile)).expect("Error saving file");

    let progfile = format!("program_{}.txt", generation);
    let _ = serialize::save_source(Path::new(&progfile), &program.simplify());
}

fn main() {
    // Generate initial random population
    println!("Generating initial population");
    let mut population = evolve::random_population(POPULATION_SIZE);
    let mut rng = rand::thread_rng();

    let mut gen = 0;
    loop {
        println!("[{}] Scoring", gen);
        gen += 1;
        population.score(score_program);
        {
            let (best_program, best_score) = population.best();
            println!("[{}] Best score: {}", gen, best_score);

            if gen % SAVE_EVERY == 0 {
                println!("[{}] Saving", gen);
                save_trace(gen, best_program);
            }
        }

        println!("[{}] Evolving", gen);
        population = population.evolve(TOURNAMENT_SIZE,
                                       10, 10, 5,
                                       &mut rng);
    }
}

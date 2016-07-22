//! Simple evolution example
#[macro_use]
extern crate ast;
extern crate rand;
extern crate rustc_serialize;

pub use rand::Rng;

use ast::structure::{Program, Number};
use ast::depth::Depth;
use ast::simulation;
use ast::serialize;
use ast::simplify::Simplify;
use ast::data::SensorData;
use ast::darwin::evolve;
use std::f32::consts::PI;
use ast::num::{square, partial_max};
use ast::darwin::evolve::{ScoreCard,OptimumKeeper};

const POPULATION_SIZE : usize = 2000;
const TRIALS_PER_PROGRAM : u32 = 10;
const TOURNAMENT_SIZE : usize = 20;

fn random_start_position<R: rand::Rng>(rng: &mut R) -> SensorData {
    SensorData::new()
        .with_x(0.0)
        .with_y(rng.next_f32() * 100.0 + 50.0)
        .with_o(rng.next_f32() * PI * 2.0)
}

/// Score a program by scoring a single run
///
/// Ultimate score is composed of:
/// - How many frames we survived (higher is better)
/// - What our maximum height was (lower is better)
/// - If we landed (if so then FUCK YEAH)
fn score_single_run<R: rand::Rng>(program: &Program, rng: &mut R) -> ScoreCard {
    let mut sensor_data = random_start_position(rng);
    let world = simulation::World::new();
    let mut trace = serialize::GameTrace::new();

    let mut total_height: Number = 0.;
    let mut total_fuel: Number = 0.;

    trace.add(&sensor_data);
    while !sensor_data.crashed && !sensor_data.landed {
        total_height += square(sensor_data.y);
        total_fuel += square(sensor_data.fuel);

        simulation::next_program(&mut sensor_data, &program, &world);
        trace.add(&sensor_data);
    };

    let frames = trace.frames() as Number;
    ScoreCard::new(vec![
        ("survival_bonus", 3.0 * frames),
        ("height_penalty", -(0.01 * total_height / frames)),
        ("fuel_bonus",     (100.0 * total_fuel / frames)),
        ("success_bonus",  if sensor_data.landed { 10000.0 } else { 0.0 })
    ], trace)
}

/// Score a program by averaging the score of multiple random runs
fn score_program<R: rand::Rng>(program: &Program, rng: &mut R) -> ScoreCard {
    let best_run = partial_max((0..TRIALS_PER_PROGRAM).map(|_| score_single_run(program, rng))).unwrap();

    // Give a penalty for program depth. Since this is the same for all
    // runs, we only do it here (for mucho saved speed!)
    best_run.add(vec![
        ("complexity_pentalty", program.depth() as f32 * -5.0)
    ])
}

fn main() {
    // Generate initial random population
    println!("Generating initial population");
    let mut population = evolve::random_population(POPULATION_SIZE);
    let mut rng = rand::thread_rng();
    let mut stdout = std::io::stdout();
    let mut keeper = OptimumKeeper::<Program>::new();

    loop {
        println!("[{}] Scoring", population.generation);
        population.score(|p| score_program(p, &mut rng));
        {
            let winner = population.winner();
            println_err!("[{}] Best score: {}", population.generation, winner.score.total_score());
            
            if keeper.improved(&winner.program, &winner.score, population.generation) {
                let _ = serialize::writeln(&population.generation, &mut stdout);
                let _ = serialize::writeln(&winner.program.simplify(), &mut stdout);
                let _ = serialize::writeln(&winner.score.trace().trace(), &mut stdout);
                let _ = serialize::writeln(&winner.score.scores(), &mut stdout);
            }
        }


        println!("[{}] Evolving", population.generation);
        population = population.evolve(TOURNAMENT_SIZE,
                                       10, 10, 5,
                                       &mut rng);
    }
}

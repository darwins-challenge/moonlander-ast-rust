//! Simple evolution example
#[macro_use]
extern crate ast;
extern crate rand;
extern crate rustc_serialize;
extern crate env_logger;

pub use rand::Rng;
use std::io::stdout;

use ast::structure::{Condition, Number};
use ast::simulation;
use ast::serialize;
use ast::depth::Depth;
use ast::simplify::Simplify;
use ast::data::SensorData;
use ast::num::{square, partial_max};
use ast::darwin::evolve;
use ast::darwin::evolve::{ScoreCard,OptimumKeeper};


const POPULATION_SIZE : usize = 2000;
const TRIALS_PER_PROGRAM : usize = 3;
const TOURNAMENT_SIZE : usize = 100;

const REPRODUCE_WEIGHT : u32 = 10;
const MUTATE_WEIGHT : u32 = 10;
const CROSSOVER_WEIGHT : u32 = 10;

fn random_start_position<R: rand::Rng>(rng: &mut R) -> SensorData {
    SensorData::new()
        .with_x(0.0)
        .with_y(rng.next_f32() * 600.0 + 50.0)
        .with_o(0.0)
}

/// Score a program by scoring a single run
///
/// Ultimate score is composed of:
/// - How many frames we survived (higher is better)
/// - What our maximum height was (lower is better)
/// - If we landed (if so then FUCK YEAH)
fn score_single_run<R: rand::Rng>(program: &Condition, rng: &mut R) -> ScoreCard {
    let mut sensor_data = random_start_position(rng);
    let world = simulation::World::new().with_max_landing_speed(0.5);
    let mut trace = serialize::GameTrace::new();

    let mut total_height: Number = 0.;
    let mut total_fuel: Number = 0.;

    trace.add(&sensor_data);
    while !sensor_data.hit_ground {
        total_height += square(sensor_data.y);
        total_fuel += square(sensor_data.fuel);

        simulation::next_condition(&mut sensor_data, &program, &world);
        trace.add(&sensor_data);
    };

    let frames = trace.frames() as Number;
    ScoreCard::new(vec![
        ("survival_bonus",   3.0 * frames),
        ("height_penalty",   -(0.01 * total_height / frames)),
        ("fuel_bonus",        (100.0 * total_fuel / frames)),
        ("hit_ground_bonus", if sensor_data.hit_ground { 10.0 } else { 0.0 }),
        ("crash_penalty",    sensor_data.crash_speed),
        ("success_bonus",    if sensor_data.landed { 10000.0 } else { 0.0 }),
    ], trace)
}

/// Score a program by averaging the score of multiple random runs
fn score_program<R: rand::Rng>(program: &Condition, rng: &mut R) -> ScoreCard {
    let best_run = partial_max((0..TRIALS_PER_PROGRAM).map(|_| score_single_run(program, rng))).unwrap();

    // Give a penalty for program depth. Since this is the same for all
    // runs, we only do it here (for mucho saved speed!)
    best_run.add(vec![
        ("complexity_pentalty", program.depth() as f32 * -5.0)
    ])
}

fn main() {
    env_logger::init().expect("Error initializing logger");

    // Generate initial random population
    println_err!("Generating initial population");
    let mut population = evolve::random_population::<Condition>(POPULATION_SIZE);
    let mut rng = rand::StdRng::new().unwrap();
    let mut stdout = std::io::stdout();
    let mut keeper = OptimumKeeper::<Condition>::new();

    loop {
        serialize::log(&population.population);
        println_err!("[{}] Scoring", population.generation);
        population.score(|p| score_program(p, &mut rng));
        {
            let winner = population.winner();
            println_err!("[{}] Best score: {}", population.generation, winner.score.total_score());
            
            if keeper.improved(&winner.program, &winner.score, population.generation) {
                let _ = serialize::writeln(&population.generation, &mut stdout);
                println!("{}", winner.program.simplify());
//                let _ = serialize::writeln(&winner.program.simplify(), &mut stdout);
                let _ = serialize::writeln(&winner.score.trace().trace(), &mut stdout);
                let _ = serialize::writeln(&winner.score.scores(), &mut stdout);
            }
        }

        println_err!("[{}] Evolving", population.generation);
        population = population.evolve(TOURNAMENT_SIZE,
                                       REPRODUCE_WEIGHT, 
                                       MUTATE_WEIGHT, 
                                       CROSSOVER_WEIGHT,
                                       &mut rng);
    }
}

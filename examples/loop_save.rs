extern crate rand;
extern crate ast;

use ast::structure::Program;
use ast::data::SensorData;
use ast::simulation::{next_program,World};
use ast::source::Source;
use ast::serialize::GameTrace;
use std::env;
use std::process;
use std::path::Path;

fn main() {
    if env::args().len() < 2 {
        println!("Usage: loop_save FILE.json");
        process::exit(1);
    }

    let filename = env::args().nth(1).unwrap();
    let program: Program = rand::random();
    let mut sensor_data: SensorData = SensorData::new().with_y(100.0);
    let world: World = World::new();
    let mut trace = GameTrace::new();

    println!("{}", program.source());
    let mut count: i32 = 0;
    loop {
        next_program(&mut sensor_data, &program, &world);
        trace.add(&sensor_data);
        count += 1;
        if sensor_data.crashed || sensor_data.landed { break; }
    }

    trace.save_file(Path::new(&filename)).expect("Error saving file");

    println!("Program lasted {} iterations, saved to {}", count, filename);
}

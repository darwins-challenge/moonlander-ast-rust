extern crate rand;
extern crate ast;

use ast::structure::Program;
use ast::data::SensorData;
use ast::simulation::{next_program,World};
use ast::source::Source;

fn main() {
    let program: Program = rand::random();
    let mut sensor_data: SensorData = SensorData::new().with_y(100.0);
    let world: World = World::new();

    println!("{}", program.source());
    let mut count: i32 = 0;
    loop {
        next_program(&mut sensor_data, &program, &world);
        count += 1;
        if sensor_data.crashed || sensor_data.landed { break; }
    }

    println!("Program lasted {} iterations", count);
}

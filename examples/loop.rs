extern crate rand;
extern crate ast;

use ast::structure::Program;
use ast::data::SensorData;
use ast::simulation::{next,World};
use ast::source::Source;

fn main() {
    let program: Program = rand::random();
    let mut sensor_data: SensorData = SensorData::new().with_y(100.0).build();
    let world: World = World {
        angular_increment: 0.1,
        gravitational_constant: -0.5,
        thrust_constant: 0.6,
        tolerance: 0.01
    };

    println!("{}", program.source());
    let mut count: i32 = 0;
    loop {
        next(&mut sensor_data, &program, &world);
        count += 1;
        if sensor_data.crashed || sensor_data.landed || count > 1000 { break; }
    }

    println!("Program lasted {} iterations", count);
}

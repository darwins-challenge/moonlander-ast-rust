use super::structure::{Program, Command, Condition, Number};
use super::data::{SensorData, Evaluate, BooleanValue};

fn abs(x: Number) -> Number {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

#[derive(Clone,Copy)]
pub struct World {
    pub angular_increment: Number,
    pub gravitational_constant: Number,
    pub thrust_constant: Number,
    pub tolerance: Number,
    pub fuel_consumption: Number,
    pub max_landing_speed: Number
}

impl World {
    /// Creates a builder for `World`. it should be used in the following sense:
    ///
    /// ```
    /// let world = ast::simulation::World::new();
    /// ```
    pub fn new() -> World {
        World {
            angular_increment: 0.1,
            gravitational_constant: -0.5,
            thrust_constant: 0.6,
            tolerance: 0.01,
            fuel_consumption: 0.01,
            max_landing_speed: 10.0
        }
    }

    pub fn with_angular_increment(self, angular_increment: Number) -> World {
        World { angular_increment: angular_increment, ..self}
    }

    pub fn with_gravitational_constant(self, gravitational_constant: Number) -> World {
        World { gravitational_constant: gravitational_constant, ..self}
    }

    pub fn with_max_landing_speed(self, max_landing_speed: Number) -> World {
        World { max_landing_speed: max_landing_speed, ..self}
    }

    pub fn with_thrust_constant(self, thrust_constant: Number) -> World {
        World { thrust_constant: thrust_constant, ..self}
    }

    pub fn with_tolerance(self, tolerance: Number) -> World {
        World { tolerance: tolerance, ..self}
    }

    pub fn with_fuel_consumption(self, fuel_consumption: Number) -> World {
        World { fuel_consumption: fuel_consumption, ..self}
    }
}

pub fn update_data(sensor_data: &mut SensorData, command: Command, world: &World) {
    if sensor_data.crashed || sensor_data.landed { return; }

    let angular_multiplier: Number = match command {
        Command::Left  =>  1.0,
        Command::Right => -1.0,
        _              =>  0.0
    };
    sensor_data.w += angular_multiplier * world.angular_increment;
    sensor_data.o += sensor_data.w;

    let thrust_multiplier: Number = match command {
        Command::Thrust => { if sensor_data.fuel > 0.0 { 1.0 } else { 0.0 } },
        _               => 0.0
    };
    let acceleration = thrust_multiplier * world.thrust_constant;
    let ax = -acceleration * sensor_data.o.sin();
    let ay = acceleration * sensor_data.o.cos() + world.gravitational_constant;
    sensor_data.vx += ax;
    sensor_data.vy += ay;
    sensor_data.x += sensor_data.vx;
    sensor_data.y += sensor_data.vy;

    sensor_data.fuel -= match command {
        Command::Thrust => world.fuel_consumption,
        _               => 0.0
    };
    sensor_data.fuel = if sensor_data.fuel > 0.0 { sensor_data.fuel } else { 0.0 };

    let down = sensor_data.y < world.tolerance;
    let upright = abs(sensor_data.o) < world.tolerance;
    sensor_data.crash_speed = abs(sensor_data.vy);
    let crashed = sensor_data.crash_speed > world.max_landing_speed;

    sensor_data.crashed = down && (!upright || crashed);
    sensor_data.landed = down && upright && !crashed;

    sensor_data.thrusting = match command {
        Command::Thrust => true,
        _               => false
    }
}

pub fn next_program(sensor_data: &mut SensorData, program: &Program, world: &World) {
    let command = program.evaluate(*sensor_data);
    update_data(sensor_data, command, world);
}

pub fn next_condition(sensor_data: &mut SensorData, cond: &Condition, world: &World) {
    let result = cond.value(*sensor_data);
    update_data(sensor_data, if result { Command::Thrust } else { Command::Skip }, world);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32;
    use data::{SensorData};
    use structure::{Program, Command};

    #[test]
    fn next_should_land_if_all_motion_is_stopped_and_near_horizon() {
        let mut sensor_data: SensorData = SensorData::new().with_vy(0.5);
        let program = Program::Command(Box::new(Command::Skip));
        let world = World::new();

        next_program(&mut sensor_data, &program, &world);

        assert!(sensor_data.landed);
    }

   #[test]
    fn next_should_increment_velocity_with_thrust_constant() {
        let mut sensor_data: SensorData = SensorData::new().with_o(-f32::consts::PI);
        let program = Program::Command(Box::new(Command::Thrust));
        let world = World::new();

        next_program(&mut sensor_data, &program, &world);

        assert!(sensor_data.vx < 0.0);
    }

    #[test]
    fn next_should_increment_position_with_velocity() {
        let mut sensor_data: SensorData = SensorData::new().with_vx(1.0).with_vy(1.0);
        let program = Program::Command(Box::new(Command::Skip));
        let world = World::new();

        next_program(&mut sensor_data, &program, &world);

        assert!(sensor_data.x > 0.0);
        assert!(sensor_data.y > 0.0);
    }

    #[test]
    fn next_should_increment_orientation_with_angular_velocity() {
        let mut sensor_data: SensorData = SensorData::new().with_w(1.0);
        let program = Program::Command(Box::new(Command::Skip));
        let world = World::new();

        next_program(&mut sensor_data, &program, &world);

        assert!(sensor_data.o > 0.0);
    }

   #[test]
    fn next_should_increment_angular_velocity_when_command_is_left() {
        let mut sensor_data: SensorData = SensorData::new();
        let program = Program::Command(Box::new(Command::Left));
        let world = World::new();

        next_program(&mut sensor_data, &program, &world);

        assert!(sensor_data.w > 0.0);
    }

    #[test]
    fn next_should_decrement_angular_velocity_when_command_is_right() {
        let mut sensor_data: SensorData = SensorData::new();
        let program = Program::Command(Box::new(Command::Right));
        let world = World::new();

        next_program(&mut sensor_data, &program, &world);

        assert!(sensor_data.w < 0.0);
    }

    #[test]
    fn next_should_signal_thursting_when_command_is_thrust() {
        let mut sensor_data: SensorData = SensorData::new();
        let program = Program::Command(Box::new(Command::Thrust));
        let world = World::new();

        next_program(&mut sensor_data, &program, &world);

        assert!(sensor_data.thrusting);
    }

    #[test]
    fn next_should_not_signal_thursting_when_command_is_other_then_thrust() {
        let mut sensor_data: SensorData = SensorData::new();
        let program = Program::Command(Box::new(Command::Skip));
        let world = World::new();

        next_program(&mut sensor_data, &program, &world);

        assert!(!sensor_data.thrusting);
    }

    #[test]
    fn next_should_consume_fuel_when_thrusting() {
        let mut sensor_data: SensorData = SensorData::new().with_fuel(1.0);
        let program = Program::Command(Box::new(Command::Thrust));
        let world = World::new().with_fuel_consumption(0.01);

        next_program(&mut sensor_data, &program, &world);

        println!("{}", sensor_data.fuel);
        assert!(sensor_data.fuel < 1.0);
    }

    #[test]
    fn next_should_not_consume_more_fuel_when_out() {
        let mut sensor_data: SensorData = SensorData::new().with_fuel(0.0);
        let program = Program::Command(Box::new(Command::Thrust));
        let world = World::new().with_fuel_consumption(0.01);

        next_program(&mut sensor_data, &program, &world);

        println!("{}", sensor_data.fuel);
        assert!(sensor_data.fuel == 0.0);
    }

   #[test]
    fn next_should_not_change_velocity_when_fuel_is_out_when_thrusting() {
        let mut sensor_data: SensorData = SensorData::new().with_vx(0.0).with_o(f32::consts::PI/2.0).with_fuel(0.0);
        let program = Program::Command(Box::new(Command::Thrust));
        let world = World::new();

        next_program(&mut sensor_data, &program, &world);

        assert!(sensor_data.vx == 0.0);
    }
}

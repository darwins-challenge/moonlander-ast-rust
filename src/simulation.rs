use super::structure::{Program, Command, Condition};
use super::data::{SensorData, Evaluate, BooleanValue};

fn abs(x: f32) -> f32 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

#[derive(Clone,Copy)]
pub struct World {
    pub angular_increment: f32,
    pub gravitational_constant: f32,
    pub thrust_constant: f32,
    pub tolerance: f32,
    pub fuel_consumption: f32,
}

impl World {
    /// Creates a builder for `World`. it should be used in the following sense:
    ///
    /// ```
    /// let world = ast::simulation::World::new().build();
    /// ```
    pub fn new() -> World {
        World {
            angular_increment: 0.1,
            gravitational_constant: -0.5,
            thrust_constant: 0.6,
            tolerance: 0.01,
            fuel_consumption: 0.01,
        }
    }

    pub fn with_angular_increment<'a>(&'a mut self, angular_increment: f32) -> &'a mut World {
        self.angular_increment = angular_increment;
        self
    }

    pub fn with_gravitational_constant<'a>(&'a mut self, gravitational_constant: f32) -> &'a mut World {
        self.gravitational_constant = gravitational_constant;
        self
    }

    pub fn with_thrust_constant<'a>(&'a mut self, thrust_constant: f32) -> &'a mut World {
        self.thrust_constant = thrust_constant;
        self
    }

    pub fn with_tolerance<'a>(&'a mut self, tolerance: f32) -> &'a mut World {
        self.tolerance = tolerance;
        self
    }

    pub fn with_fuel_consumption<'a>(&'a mut self, fuel_consumption: f32) -> &'a mut World {
        self.fuel_consumption = fuel_consumption;
        self
    }

    pub fn build<'a>(&'a mut self) -> World {
        *self
    }
}

pub fn update_data(sensor_data: &mut SensorData, command: Command, world: &World) {
    if sensor_data.crashed || sensor_data.landed { return; }

    let angular_multiplier: f32 = match command {
        Command::Left  =>  1.0,
        Command::Right => -1.0,
        _              =>  0.0
    };
    sensor_data.w += angular_multiplier * world.angular_increment;
    sensor_data.o += sensor_data.w;

    let thrust_multiplier: f32 = match command {
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

    sensor_data.crashed = sensor_data.y < -world.tolerance ||
        (abs(sensor_data.y) < world.tolerance &&
         (sensor_data.vy < -world.tolerance ||
          abs(sensor_data.vx) > world.tolerance ||
          abs(sensor_data.o) > world.tolerance ||
          abs(sensor_data.w) > world.tolerance));

    sensor_data.landed = !sensor_data.crashed &&
        abs(sensor_data.y) < world.tolerance &&
        abs(sensor_data.vy) < world.tolerance &&
        abs(sensor_data.vx) < world.tolerance &&
        abs(sensor_data.o) < world.tolerance &&
        abs(sensor_data.w) < world.tolerance;

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
        let mut sensor_data: SensorData = SensorData::new().with_vy(0.5).build();
        let program = Program::Command(Box::new(Command::Skip));
        let world = World::new().build();

        next(&mut sensor_data, &program, &world);

        assert!(sensor_data.landed);
    }

   #[test]
    fn next_should_increment_velocity_with_thrust_constant() {
        let mut sensor_data: SensorData = SensorData::new().with_o(-f32::consts::PI).build();
        let program = Program::Command(Box::new(Command::Thrust));
        let world = World::new().build();

        next(&mut sensor_data, &program, &world);

        assert!(sensor_data.vx < 0.0);
    }

    #[test]
    fn next_should_increment_position_with_velocity() {
        let mut sensor_data: SensorData = SensorData::new().with_vx(1.0).with_vy(1.0).build();
        let program = Program::Command(Box::new(Command::Skip));
        let world = World::new().build();

        next(&mut sensor_data, &program, &world);

        assert!(sensor_data.x > 0.0);
        assert!(sensor_data.y > 0.0);
    }

    #[test]
    fn next_should_increment_orientation_with_angular_velocity() {
        let mut sensor_data: SensorData = SensorData::new().with_w(1.0).build();
        let program = Program::Command(Box::new(Command::Skip));
        let world = World::new().build();

        next(&mut sensor_data, &program, &world);

        assert!(sensor_data.o > 0.0);
    }

   #[test]
    fn next_should_increment_angular_velocity_when_command_is_left() {
        let mut sensor_data: SensorData = SensorData::new().build();
        let program = Program::Command(Box::new(Command::Left));
        let world = World::new().build();

        next(&mut sensor_data, &program, &world);

        assert!(sensor_data.w > 0.0);
    }

    #[test]
    fn next_should_decrement_angular_velocity_when_command_is_right() {
        let mut sensor_data: SensorData = SensorData::new().build();
        let program = Program::Command(Box::new(Command::Right));
        let world = World::new().build();

        next(&mut sensor_data, &program, &world);

        assert!(sensor_data.w < 0.0);
    }

    #[test]
    fn next_should_signal_thursting_when_command_is_thrust() {
        let mut sensor_data: SensorData = SensorData::new().build();
        let program = Program::Command(Box::new(Command::Thrust));
        let world = World::new().build();

        next(&mut sensor_data, &program, &world);

        assert!(sensor_data.thrusting);
    }

    #[test]
    fn next_should_not_signal_thursting_when_command_is_other_then_thrust() {
        let mut sensor_data: SensorData = SensorData::new().build();
        let program = Program::Command(Box::new(Command::Skip));
        let world = World::new().build();

        next(&mut sensor_data, &program, &world);

        assert!(!sensor_data.thrusting);
    }

    #[test]
    fn next_should_consume_fuel_when_thrusting() {
        let mut sensor_data: SensorData = SensorData::new().with_fuel(1.0).build();
        let program = Program::Command(Box::new(Command::Thrust));
        let world = World::new().with_fuel_consumption(0.01).build();

        next(&mut sensor_data, &program, &world);

        println!("{}", sensor_data.fuel);
        assert!(sensor_data.fuel < 1.0);
    }

    #[test]
    fn next_should_not_consume_more_fuel_when_out() {
        let mut sensor_data: SensorData = SensorData::new().with_fuel(0.0).build();
        let program = Program::Command(Box::new(Command::Thrust));
        let world = World::new().with_fuel_consumption(0.01).build();

        next(&mut sensor_data, &program, &world);

        println!("{}", sensor_data.fuel);
        assert!(sensor_data.fuel == 0.0);
    }

   #[test]
    fn next_should_not_change_velocity_when_fuel_is_out_when_thrusting() {
        let mut sensor_data: SensorData = SensorData::new().with_vx(0.0).with_o(f32::consts::PI/2.0).with_fuel(0.0).build();
        let program = Program::Command(Box::new(Command::Thrust));
        let world = World::new().build();

        next(&mut sensor_data, &program, &world);

        assert!(sensor_data.vx == 0.0);
    }
}

use super::structure::{Program, Command};
use super::data::{SensorData, Evaluate};

fn abs(x: f32) -> f32 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

pub struct World {
    pub angular_increment: f32,
    pub gravitational_constant: f32,
    pub thrust_constant: f32,
    pub tolerance: f32
}

pub fn next(sensor_data: &mut SensorData, program: Program, world: World) {
    if sensor_data.crashed || sensor_data.landed { return; }
    let command = program.evaluate(*sensor_data);

    let angular_multiplier: f32 = match **command {
        Command::Left  =>  1.0,
        Command::Right => -1.0,
        _              =>  0.0
    };
    sensor_data.w += angular_multiplier * world.angular_increment;
    sensor_data.o += sensor_data.w;

    let thrust_multiplier: f32 = match **command {
        Command::Thrust => 1.0,
        _               => 0.0
    };
    let acceleration = thrust_multiplier * world.thrust_constant;
    let ax = -acceleration * sensor_data.o.sin();
    let ay = acceleration * sensor_data.o.cos() + world.gravitational_constant;
    sensor_data.vx += ax;
    sensor_data.vy += ay;
    sensor_data.x += sensor_data.vx;
    sensor_data.y += sensor_data.vy;

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

    sensor_data.thrusting = match **command {
        Command::Thrust => true,
        _               => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32;
    use data::{SensorData};
    use structure::{Program, Command};

    #[test]
    fn next_should_land_if_all_motion_is_stopped_and_near_horizon() {
        let mut sensor_data: SensorData = SensorData { x: 0.0, y: 0.0, vx: 0.0, vy: 0.5, o: 0.0, w: 0.0, crashed: false, landed: false, thrusting: false };
        let program = Program::Command(Command::Skip);
        let world = World { angular_increment: 0.1, gravitational_constant: -0.5, thrust_constant: 0.6, tolerance: 0.0001 };

        next(&mut sensor_data, program, world);

        assert!(sensor_data.landed);
    }

   #[test]
    fn next_should_increment_velocity_with_thrust_constant() {
        let mut sensor_data: SensorData = SensorData { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0, o: f32::consts::PI/2.0, w: 0.0, crashed: false, landed: false, thrusting: false };
        let program = Program::Command(Command::Thrust);
        let world = World { angular_increment: 0.1, gravitational_constant: -0.5, thrust_constant: 0.6, tolerance: 0.0001 };

        next(&mut sensor_data, program, world);

        assert!(sensor_data.vx < 0.0);
    }

    #[test]
    fn next_should_increment_position_with_velocity() {
        let mut sensor_data: SensorData = SensorData { x: 0.0, y: 0.0, vx: 1.0, vy: 1.0, o: 0.0, w: 0.0, crashed: false, landed: false, thrusting: false };
        let program = Program::Command(Command::Skip);
        let world = World { angular_increment: 0.1, gravitational_constant: -0.5, thrust_constant: 0.6, tolerance: 0.0001 };

        next(&mut sensor_data, program, world);

        assert!(sensor_data.x > 0.0);
        assert!(sensor_data.y > 0.0);
    }

    #[test]
    fn next_should_increment_orientation_with_angular_velocity() {
        let mut sensor_data: SensorData = SensorData { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0, o: 0.0, w: 1.0, crashed: false, landed: false, thrusting: false };
        let program = Program::Command(Command::Skip);
        let world = World { angular_increment: 0.1, gravitational_constant: -0.5, thrust_constant: 0.6, tolerance: 0.0001 };

        next(&mut sensor_data, program, world);

        assert!(sensor_data.o > 0.0);
    }

   #[test]
    fn next_should_increment_angular_velocity_when_command_is_left() {
        let mut sensor_data: SensorData = SensorData { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0, o: 0.0, w: 0.0, crashed: false, landed: false, thrusting: false };
        let program = Program::Command(Command::Left);
        let world = World { angular_increment: 0.1, gravitational_constant: -0.5, thrust_constant: 0.6, tolerance: 0.0001 };

        next(&mut sensor_data, program, world);

        assert!(sensor_data.w > 0.0);
    }

    #[test]
    fn next_should_decrement_angular_velocity_when_command_is_right() {
        let mut sensor_data: SensorData = SensorData { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0, o: 0.0, w: 0.0, crashed: false, landed: false, thrusting: false };
        let program = Program::Command(Command::Right);
        let world = World { angular_increment: 0.1, gravitational_constant: -0.5, thrust_constant: 0.6, tolerance: 0.0001  };

        next(&mut sensor_data, program, world);

        assert!(sensor_data.w < 0.0);
    }

    #[test]
    fn next_should_signal_thursting_when_command_is_thrust() {
        let mut sensor_data: SensorData = SensorData { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0, o: 0.0, w: 0.0, crashed: false, landed: false, thrusting: false };
        let program = Program::Command(Command::Thrust);
        let world = World { angular_increment: 0.1, gravitational_constant: -0.5, thrust_constant: 0.6, tolerance: 0.0001  };

        next(&mut sensor_data, program, world);

        assert!(sensor_data.thrusting);
    }

    #[test]
    fn next_should_not_signal_thursting_when_command_is_other_then_thrust() {
        let mut sensor_data: SensorData = SensorData { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0, o: 0.0, w: 0.0, crashed: false, landed: false, thrusting: false };
        let program = Program::Command(Command::Skip);
        let world = World { angular_increment: 0.1, gravitational_constant: -0.5, thrust_constant: 0.6, tolerance: 0.0001  };

        next(&mut sensor_data, program, world);

        assert!(!sensor_data.thrusting);
    }
}

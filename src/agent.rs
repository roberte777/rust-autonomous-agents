use crate::physics::{Position, Vector};
use rand::Rng;

pub struct Agent {
    pub pos: Position,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    velocity: Vector,
    acceleration: Vector,
    max_speed: Vector,
    max_force: Vector,
    pub desired: Vector,
}

impl Agent {
    pub fn new(size: (u32, u32)) -> Agent {
        let max_x = (size.0 as f32 / 2.0).floor();
        let max_y = (size.1 as f32 / 2.0).floor();
        let mut rng = rand::thread_rng();
        println!("{} {}", max_x, max_y);
        let x = rng.gen_range(-max_x..max_x);
        let y = rng.gen_range(-max_y..max_y);
        println!("{} {}", x, y);
        return Agent {
            pos: Position { x, y },
            width: 20.0,
            height: 20.0,
            rotation: 0.0,
            velocity: Vector { x: 0.0, y: 0.0 },
            acceleration: Vector { x: 0.0, y: 0.0 },
            max_speed: Vector { x: 2.0, y: 2.0 },
            max_force: Vector { x: 5.0, y: 5.0 },
            desired: Vector { x: 0.0, y: 0.0 },
        };
    }

    pub fn update_angle(&mut self) {
        self.rotation = self.velocity.y.atan2(self.velocity.x);
    }

    pub fn move_agent(&mut self) {
        let mut new_vel = self.velocity + self.acceleration;
        new_vel.adjust_to_max(self.max_speed);
        self.velocity = new_vel;
        println!("{} {}", self.velocity.x, self.velocity.y);
        self.pos.x = self.pos.x + self.velocity.x;
        self.pos.y = self.pos.y + self.velocity.y;
        self.acceleration.x = 0.0;
        self.acceleration.y = 0.0;
        self.update_angle();
    }
    pub fn steer(&mut self, target: &Position) {
        // println!("{} {}", self.pos.x, self.pos.y);
        let mut desired = Vector {
            x: target.x - self.pos.x,
            y: target.y - self.pos.y,
        };
        self.desired = desired;
        let mag = (self.max_speed.x.powi(2) + self.max_speed.y.powi(2)).sqrt();
        desired.set_mag(mag, desired.y.atan2(desired.x));
        let mut steer_vector = desired - self.velocity;
        steer_vector.adjust_to_max(self.max_force);
        self.acceleration = steer_vector;

        self.move_agent();
    }
}

use crate::physics::{Position, Vector};
use rand::Rng;

struct WorldSize {
    max_x: f32,
    min_x: f32,
    max_y: f32,
    min_y: f32,
}

pub struct Agent {
    pub pos: Position,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    world_size: WorldSize,
    velocity: Vector,
    acceleration: Vector,
    max_speed: f32,
    max_force: f32,
    pub desired: Vector,
}

impl Agent {
    pub fn new(size: (u32, u32)) -> Agent {
        let max_x = (size.0 as f32 / 2.0).floor();
        let max_y = (size.1 as f32 / 2.0).floor();
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-max_x..max_x);
        let y = rng.gen_range(-max_y..max_y);
        return Agent {
            pos: Position { x, y },
            width: 20.0,
            height: 20.0,
            rotation: 0.0,
            velocity: Vector { x: 0.0, y: 0.0 },
            acceleration: Vector { x: 0.0, y: 0.0 },
            max_speed: 4.0,
            max_force: 0.15,
            world_size: WorldSize {
                max_x,
                max_y,
                min_x: -max_x,
                min_y: -max_y,
            },
            desired: Vector { x: 0.0, y: 0.0 },
        };
    }
    pub fn edges(&mut self) {
        //if x off the right of the screen, wrap around to left
        if self.pos.x > self.world_size.max_x {
            self.pos.x = self.world_size.min_x;
        }
        //if x off left side of screen, wrap to right
        else if self.pos.x < self.world_size.min_x {
            self.pos.x = self.world_size.max_x;
        }
        //if off top of screen, wrap to bottom
        if self.pos.y > self.world_size.max_y {
            self.pos.y = self.world_size.min_y;
        }
        //if off bottom of screen, wrap to top
        else if self.pos.y < self.world_size.min_y {
            self.pos.y = self.world_size.max_y;
        }
    }
    //applies the force to the agents acceleration, reason it moves
    pub fn apply_force(&mut self, force: Vector) {
        self.acceleration = self.acceleration + force;
    }
    //makes the agent point the direction it is going
    pub fn update_angle(&mut self) {
        self.rotation = self.velocity.y.atan2(self.velocity.x);
    }
    //movest the agent each frame according to its physics
    pub fn move_agent(&mut self) {
        //determine the velocity after adding acceleration
        let mut new_vel = self.velocity + self.acceleration;
        //limit the new velocity to the max speed
        new_vel.limit(self.max_speed);
        //update velocity of the agent
        self.velocity = new_vel;
        //update position from the velocity
        self.pos.x = self.pos.x + self.velocity.x;
        self.pos.y = self.pos.y + self.velocity.y;
        //reset acceleration
        self.acceleration.x = 0.0;
        self.acceleration.y = 0.0;
        //update the angle so the agent is pointin where it is going
        self.update_angle();
    }
    //Steers the agent AWAY from a target
    pub fn flee(&mut self, target: &Position) -> Vector {
        return self.seek(target) * -1.0;
    }
    //Steers the agent towards its target
    pub fn seek(&mut self, target: &Position) -> Vector {
        //desired vector is a straight line to its target.
        let mut desired = Vector {
            x: target.x - self.pos.x,
            y: target.y - self.pos.y,
        };
        //giving this to the agent so I can draw it
        self.desired = desired;
        //setting the desired vector to the max max_speed
        //because the agent wants to reach target as fast as possible
        desired.set_mag(self.max_speed);
        //calculation for steering force is desired - current velocity
        let mut steer_vector = desired - self.velocity;
        //make sure force is not bigger than max force
        steer_vector.limit(self.max_force);
        return steer_vector;
    }
    pub fn pursue(&mut self, target_agent: Agent) -> Vector {
        let mut target = target_agent.pos;
        target.x = target.x + target_agent.velocity.x;
        target.y = target.y + target_agent.velocity.y;
        return self.seek(&target);
    }
}

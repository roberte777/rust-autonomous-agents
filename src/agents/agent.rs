use crate::physics::{Position, Vector};

#[derive(Clone, Copy)]
pub struct AgentState {
    pub pos: Position,
    pub rotation: f32,
    pub velocity: Vector,
    pub acceleration: Vector,
    pub max_speed: f32,
    pub max_force: f32,
}

impl AgentState {
    pub fn edges(&mut self, size: (u32, u32)) {
        let top = ((size.1 as f32) / 2.0).floor();
        let right = ((size.0 as f32) / 2.0).floor();
        //if x off the right of the screen, wrap around to left
        if self.pos.x > right {
            self.pos.x = -right;
        }
        //if x off left side of screen, wrap to right
        else if self.pos.x < -right {
            self.pos.x = right;
        }
        //if off top of screen, wrap to bottom
        if self.pos.y > top {
            self.pos.y = -top;
        }
        //if off bottom of screen, wrap to top
        else if self.pos.y < -top {
            self.pos.y = top;
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
    pub fn apply_physics(&mut self) {
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
}

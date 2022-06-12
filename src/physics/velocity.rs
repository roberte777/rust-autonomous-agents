use std::ops::{Mul, Sub};

use nannou::prelude::Vec2;

#[derive(Clone, Copy)]
pub struct Velocity {
    pub x_speed: i32,
    pub y_speed: i32,
}
impl Sub for Velocity {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x_speed: self.x_speed - other.x_speed,
            y_speed: self.y_speed - other.y_speed,
        }
    }
}
//HELPER FUNCTIONS
pub fn set_velocity(max_vel: f32, min_vel: f32, vel: &Vec2) -> Vec2 {
    let vel_len = vel.length_squared();

    let mut new_vel = vel.clone();

    if vel_len > max_vel * max_vel {
        new_vel = vel.normalize_or_zero();
        new_vel = new_vel.mul(max_vel);
    } else if vel_len < min_vel * min_vel {
        new_vel = vel.normalize_or_zero();
        new_vel = new_vel.mul(min_vel);
    }
    new_vel
}

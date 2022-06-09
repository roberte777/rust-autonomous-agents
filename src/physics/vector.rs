use std::ops::{Add, Sub};

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}
impl Vector {
    pub fn adjust_to_max(&mut self, max: Vector) {
        if self.x > max.x {
            self.x = max.x;
        }
        if self.y > max.y {
            self.y = max.y;
        }
        if self.x < -max.x {
            self.x = -max.x;
        }
        if self.y < -max.y {
            self.y = -max.y;
        }
    }

    pub fn set_mag(&mut self, mag: f32, rotation: f32) {
        self.x = mag * rotation.cos();
        self.y = mag * rotation.sin();
    }
}
impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

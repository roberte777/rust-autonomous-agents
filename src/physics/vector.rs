use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}
impl Vector {
    ///Takes a vector that is supposed to represent the maximum maginitude
    ///your vector can have, and makes your vector smaller if it has a
    ///magnitude that is greater than than max value
    pub fn limit(&mut self, max: f32) {
        if self.magnitude() > max {
            self.set_mag(max)
        }
    }

    pub fn set_mag(&mut self, mag: f32) {
        let rotation = self.y.atan2(self.x);
        self.x = mag * rotation.cos();
        self.y = mag * rotation.sin();
    }
    pub fn magnitude(&self) -> f32 {
        return ((self.x * self.x) + (self.y * self.y)).sqrt();
    }
}
impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        return Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let x = self.x * rhs;
        let y = self.y * rhs;
        return Self { x, y };
    }
}
impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        let x = self.x / rhs;
        let y = self.y / rhs;
        return Self { x, y };
    }
}

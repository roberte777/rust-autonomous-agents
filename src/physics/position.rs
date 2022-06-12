use std::ops::{Add, Div, Mul, Sub};

use super::Vector;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Position {
    pub fn distance(&self, input_pos: &Position) -> f32 {
        let x = (input_pos.x - self.x).powi(2);
        let y = (input_pos.x - self.x).powi(2);
        return (x + y).sqrt();
    }
}
impl Add for Position {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
impl Sub for Position {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        return Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Mul<f32> for Position {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let x = self.x * rhs;
        let y = self.y * rhs;
        return Self { x, y };
    }
}
impl Div<f32> for Position {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        let x = self.x / rhs;
        let y = self.y / rhs;
        return Self { x, y };
    }
}

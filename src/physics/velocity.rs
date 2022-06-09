use std::ops::Sub;

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

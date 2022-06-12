use nannou::{color::rgb, prelude::*};

use crate::physics::{Position, Vector};

use super::{brain::flee, AgentState};

pub struct Fleer {
    pub state: AgentState,
    size: (u32, u32),
    pub color: rgb::Srgb<u8>,
    pub width: f32,
    pub height: f32,
}
impl Fleer {
    pub fn new(initial_position: Position, size: (u32, u32)) -> Fleer {
        let state = AgentState {
            pos: initial_position,
            rotation: 0.0,
            velocity: Vector { x: 5.0, y: 5.0 },
            acceleration: Vector { x: 0.0, y: 0.0 },
            max_speed: 4.0,
            max_force: 0.15,
        };
        return Fleer {
            state,
            size,
            color: STEELBLUE,
            width: 10.0,
            height: 15.0,
        };
    }
    pub fn move_agent(&mut self, target: &Position) {
        let vector = flee(&self.state, target);
        self.state.apply_force(vector);
        self.state.apply_physics();
        self.state.edges(self.size);
    }
    pub fn draw(&self, draw: &Draw) {
        draw.tri()
            .xy(vec2(self.state.pos.x as f32, self.state.pos.y as f32))
            .w_h(self.width, self.height)
            .rotate(self.state.rotation)
            .color(self.color);
    }
}

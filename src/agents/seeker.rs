use nannou::{color::rgb, prelude::*};

use crate::physics::{Position, Vector};

use super::{brain::seek, AgentState};

pub struct Seeker {
    pub state: AgentState,
    pub color: rgb::Srgb<u8>,
    pub width: f32,
    pub height: f32,
}
impl Seeker {
    pub fn new(initial_position: Position) -> Seeker {
        let state = AgentState {
            pos: initial_position,
            rotation: 0.0,
            velocity: Vector { x: 5.0, y: 5.0 },
            acceleration: Vector { x: 0.0, y: 0.0 },
            max_speed: 4.0,
            max_force: 0.15,
        };
        return Seeker {
            state,
            color: STEELBLUE,
            width: 10.0,
            height: 15.0,
        };
    }
    pub fn move_agent(&mut self, target: &Position) {
        let vector = seek(&self.state, target);
        self.state.apply_force(vector);
        self.state.apply_physics();
    }
    pub fn draw(&self, draw: &Draw) {
        draw.tri()
            .xy(vec2(self.state.pos.x as f32, self.state.pos.y as f32))
            .w_h(self.width, self.height)
            .rotate(self.state.rotation)
            .color(self.color);
    }
}

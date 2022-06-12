use nannou::prelude::*;
use nannou::Draw;

use crate::physics::Position;
use crate::physics::Vector;

use super::AgentState;

pub struct BasicBoid {
    pub state: AgentState,
    size: (u32, u32),
    width: f32,
    height: f32,
    color: rgb::Srgb<u8>,
}
impl BasicBoid {
    pub fn new(initial_position: Position, size: (u32, u32)) -> BasicBoid {
        let state = AgentState {
            pos: initial_position,
            rotation: 0.0,
            velocity: Vector { x: 5.0, y: 5.0 },
            acceleration: Vector { x: 0.0, y: 0.0 },
            max_speed: 4.0,
            max_force: 0.15,
        };
        return BasicBoid {
            state,
            size,
            color: STEELBLUE,
            width: 10.0,
            height: 15.0,
        };
    }
    pub fn move_agent(&mut self) {
        self.state.apply_physics();
        self.state.edges(self.size)
    }
    pub fn draw(&self, draw: &Draw) {
        draw.tri()
            .xy(vec2(self.state.pos.x as f32, self.state.pos.y as f32))
            .w_h(self.width, self.height)
            .rotate(self.state.rotation)
            .color(self.color);
    }
}

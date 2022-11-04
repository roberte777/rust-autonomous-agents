use nannou::{color::rgb, prelude::*};

use super::PhysicsState;
#[derive(Clone)]
pub struct Prey {
    pub state: PhysicsState,
    pub world_size: (u32, u32),
    pub color: rgb::Srgb<u8>,
    pub width: f32,
    pub height: f32,
    pub local_prey: Vec<Prey>,
    pub perception: f32,
}
impl Prey {
    pub fn new(initial_position: Vec2, world_size: (u32, u32)) -> Prey {
        let state = PhysicsState {
            pos: initial_position,
            rotation: 0.0,
            velocity: vec2(5.0, 5.0),
            acceleration: vec2(0.0, 0.0),
            max_speed: 4.0,
            min_speed: 1.0,
            max_force: 0.5,
        };
        return Prey {
            state,
            world_size,
            color: STEELBLUE,
            width: 20.0,
            height: 10.0,
            local_prey: vec![],
            perception: 100.0,
        };
    }
    pub fn move_agent(&mut self, v: Vec2) {
        self.state.apply_force(v);
        self.state.apply_physics();
        self.state.edges(self.world_size);
    }
    pub fn draw(&self, draw: &Draw) {
        draw.tri()
            .xy(vec2(self.state.pos.x as f32, self.state.pos.y as f32))
            .w_h(self.width, self.height)
            .rotate(self.state.rotation)
            .color(self.color);
    }
}

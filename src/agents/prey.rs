use nannou::{color::rgb, prelude::*};

use crate::physics::{Position, Vector};

use super::{
    brain::{alignment, cohesion, separation},
    AgentState,
};
#[derive(Clone)]
pub struct Prey {
    pub state: AgentState,
    pub world_size: (u32, u32),
    pub color: rgb::Srgb<u8>,
    pub width: f32,
    pub height: f32,
    pub local_prey: Vec<Prey>,
    pub perception: f32,
}
impl Prey {
    pub fn new(initial_position: Position, world_size: (u32, u32)) -> Prey {
        let state = AgentState {
            pos: initial_position,
            rotation: 0.0,
            velocity: Vector { x: 5.0, y: 5.0 },
            acceleration: Vector { x: 0.0, y: 0.0 },
            max_speed: 4.0,
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
    pub fn move_agent(&mut self, v: Vector) {
        // let alignment_vec = alignment(&self.state, &self.local_prey);
        // let cohesion_vec = cohesion(&self.state, &self.local_prey);
        // let separation_vec = separation(&self.state, &self.local_prey);
        // let vector = alignment_vec + cohesion_vec + separation_vec;
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
    pub fn local_boids<'a>(&self, all_boids: &'a Vec<Prey>, boid_index: usize) -> Vec<&'a Prey> {
        let mut local_boids = Vec::new();

        for i in 0..all_boids.len() {
            if i != boid_index {
                let distance = self.state.pos.distance(&all_boids[i].state.pos);
                if distance <= self.perception {
                    local_boids.push(&all_boids[i]);
                }
            }
        }

        local_boids
    }
}

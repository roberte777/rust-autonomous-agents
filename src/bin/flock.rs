use autonomous_agents::{
    agents::{
        brain::{alignment, cohesion, separation},
        Prey,
    },
    simulation::generate_random_pos,
};
use nannou::prelude::*;

const SIZE: (u32, u32) = (1300, 750);
const PREY_COUNT: u32 = 50;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    size: (u32, u32),
    boids: Vec<Prey>,
}
fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(SIZE.0, SIZE.1)
        .view(view)
        .build()
        .unwrap();
    let mut boids = vec![];
    for _ in 0..PREY_COUNT {
        let pos = generate_random_pos(SIZE);
        let agent = Prey::new(vec2(pos.x, pos.y), SIZE);
        boids.push(agent)
    }

    Model {
        _window,
        size: SIZE,
        boids,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let len = model.boids.len();
    for i in 0..len {
        let mut neighbors = vec![];
        for j in 0..len {
            let possible_neighbor = &model.boids[j];
            if i == j {
                continue;
            };
            let state = vec2(model.boids[i].state.pos.x, model.boids[i].state.pos.y);
            let other = vec2(possible_neighbor.state.pos.x, possible_neighbor.state.pos.y);
            let distance = state.distance(other);
            if distance <= model.boids[i].perception {
                println!("perception: {}", model.boids[i].perception);
                neighbors.push(possible_neighbor);
            }
        }
        // let boid = &mut model.boids[i];
        // boid.local_prey = neighbors;
        let v1 = alignment(&model.boids[i].state, &neighbors);
        let v2 = cohesion(&model.boids[i].state, &neighbors);
        let v3 = separation(&model.boids[i].state, &neighbors);
        let v4 = v1 + v2 + v3;
        model.boids[i].move_agent(v4);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for boid in &model.boids {
        boid.draw(&draw)
    }
    draw.to_frame(app, &frame).unwrap();
}

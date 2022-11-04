use autonomous_agents::{
    agents::{
        brain::{alignment, cohesion, separation},
        Prey,
    },
    simulation::generate_random_pos,
};
use nannou::prelude::*;

const SIZE: (u32, u32) = (1300, 750);
const PREY_COUNT: u32 = 100;

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
    let slice = &mut model.boids[..];
    for i in 0..slice.len() {
        if i == slice.len() - 1 {
            break;
        }
        let (mid, tail) = slice.split_at_mut(i + 1);
        let (head, element) = mid.split_at_mut(i);
        let neighbors = head
            .iter()
            .chain(tail.iter())
            .filter(|other| {
                let state = vec2(element[0].state.pos.x, element[0].state.pos.y);
                let other = vec2(other.state.pos.x, other.state.pos.y);
                let distance = state.distance(other);
                distance <= element[0].perception
            })
            .collect::<Vec<&Prey>>();
        let v1 = alignment(&element[0].state, &neighbors);
        let v2 = cohesion(&element[0].state, &neighbors);
        let v3 = separation(&element[0].state, &neighbors);
        let v4 = v1 + v2 + v3;
        element[0].move_agent(v4);
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

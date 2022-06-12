use autonomous_agents::{
    agents::{
        brain::{alignment, calculate_pursue_target, cohesion, separation},
        Prey,
    },
    simulation::generate_random_pos,
};
use nannou::prelude::*;

fn main() {
    const SIZE: (u32, u32) = (1300, 750);
    nannou::app(|app| model(app, Some(SIZE)))
        .update(update)
        .run();
}

struct Model {
    _window: window::Id,
    size: (u32, u32),
    boids: Vec<Prey>,
}
fn model(app: &App, size: Option<(u32, u32)>) -> Model {
    let size: (u32, u32) = size.unwrap_or((500, 500));
    let _window = app
        .new_window()
        .size(size.0, size.1)
        .view(view)
        .build()
        .unwrap();
    let mut boids = vec![];
    for _ in 0..10 {
        let pos = generate_random_pos(size);
        let agent = Prey::new(pos, size);
        boids.push(agent)
    }

    Model {
        _window,
        size,
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
            let distance = model.boids[i]
                .state
                .pos
                .distance(&possible_neighbor.state.pos);
            println!("{}", distance);
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
        let v5 = v2 + v3;
        println!("{:?}", v2);
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

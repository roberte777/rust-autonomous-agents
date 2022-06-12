use autonomous_agents::agents::BasicBoid;
use autonomous_agents::simulation::generate_random_pos;
use nannou::prelude::*;

fn main() {
    const SIZE: (u32, u32) = (1000, 750);
    nannou::app(|app| model(app, Some(SIZE)))
        .update(update)
        .run();
}

struct Model {
    _window: window::Id,
    agents: Vec<BasicBoid>,
}
fn model(app: &App, size: Option<(u32, u32)>) -> Model {
    let size: (u32, u32) = size.unwrap_or((500, 500));
    let _window = app
        .new_window()
        .size(size.0, size.1)
        .view(view)
        .build()
        .unwrap();

    let mut agents: Vec<BasicBoid> = vec![];
    for _ in 0..3 {
        let pos = generate_random_pos(size);
        agents.push(BasicBoid::new(pos, size));
    }

    Model { _window, agents }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    for agent in &mut _model.agents {
        agent.move_agent();
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for agent in &_model.agents {
        agent.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}

use autonomous_agents::agent::Agent;
use nannou::prelude::*;

fn main() {
    const SIZE: (u32, u32) = (1000, 750);
    nannou::app(|app| model(app, Some(SIZE)))
        .update(update)
        .run();
}

struct Model {
    _window: window::Id,
    size: (u32, u32),
    agents: Vec<Agent>,
}
fn model(app: &App, size: Option<(u32, u32)>) -> Model {
    let size = size.unwrap_or((500, 500));
    let _window = app
        .new_window()
        .size(size.0, size.1)
        .view(view)
        .build()
        .unwrap();

    let mut agents: Vec<Agent> = vec![];
    for _ in 0..3 {
        agents.push(Agent::new(size))
    }

    Model {
        _window,
        size,
        agents,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for agent in &_model.agents {
        draw.tri()
            .xy(vec2(agent.pos.x as f32, agent.pos.y as f32))
            .w_h(agent.width, agent.height)
            .z_degrees(agent.rotation)
            .color(STEELBLUE);
    }
    draw.to_frame(app, &frame).unwrap();
}

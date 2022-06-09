use autonomous_agents::agent::Agent;
use autonomous_agents::physics::Position;
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
    target: Position,
}
fn model(app: &App, size: Option<(u32, u32)>) -> Model {
    let size: (u32, u32) = size.unwrap_or((500, 500));
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
    let target = Position { x: 0.0, y: 0.0 };

    Model {
        _window,
        size,
        agents,
        target,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    for agent in &mut _model.agents {
        agent.steer(&_model.target);
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for agent in &_model.agents {
        draw.tri()
            .xy(vec2(agent.pos.x as f32, agent.pos.y as f32))
            .w_h(agent.width, agent.height)
            .rotate(agent.rotation)
            .color(STEELBLUE);
        draw.line()
            .start(vec2(agent.pos.x, agent.pos.y))
            .end(vec2(
                agent.pos.x + agent.desired.x,
                agent.pos.y + agent.desired.y,
            ))
            .color(WHITE);
    }
    draw.ellipse()
        .x_y(_model.target.x as f32, _model.target.y as f32)
        .radius(10.0);
    draw.to_frame(app, &frame).unwrap();
}

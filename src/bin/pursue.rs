use autonomous_agents::agents::Agent;
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
    agent: Agent,
    pursuer: Agent,
}
fn model(app: &App, size: Option<(u32, u32)>) -> Model {
    let size: (u32, u32) = size.unwrap_or((500, 500));
    let _window = app
        .new_window()
        .size(size.0, size.1)
        .view(view)
        .build()
        .unwrap();

    let agent = Agent::new(size);
    let pursuer = Agent::new(size);

    Model {
        _window,
        size,
        agent,
        pursuer,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let steer_force = model.pursuer.seek(&model.agent.pos);
    model.pursuer.apply_force(steer_force);
    model.pursuer.move_agent();
    let steer_force = model.agent.flee(&model.pursuer.pos);
    model.agent.apply_force(steer_force);
    model.agent.move_agent();
    model.agent.edges();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let agent = &model.agent;
    let pursuer = &model.pursuer;
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.tri()
        .xy(vec2(agent.pos.x as f32, agent.pos.y as f32))
        .w_h(agent.width, agent.height)
        .rotate(agent.rotation)
        .color(STEELBLUE);
    draw.tri()
        .xy(vec2(pursuer.pos.x as f32, pursuer.pos.y as f32))
        .w_h(pursuer.width, pursuer.height)
        .rotate(pursuer.rotation)
        .color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
}

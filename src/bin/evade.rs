use autonomous_agents::{
    agents::{brain::calculate_pursue_target, BasicBoid, Evader},
    simulation::generate_random_pos,
};
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
    agent: BasicBoid,
    pursuer: Evader,
}
fn model(app: &App, size: Option<(u32, u32)>) -> Model {
    let size: (u32, u32) = size.unwrap_or((500, 500));
    let _window = app
        .new_window()
        .size(size.0, size.1)
        .view(view)
        .build()
        .unwrap();

    let pos = generate_random_pos(size);
    let agent = BasicBoid::new(pos, size);
    let pos = generate_random_pos(size);
    let pursuer = Evader::new(pos, size);

    Model {
        _window,
        size,
        agent,
        pursuer,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.agent.move_agent();
    model.pursuer.move_agent(&model.agent.state);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let agent = &model.agent;
    let pursuer = &model.pursuer;
    let draw = app.draw();
    draw.background().color(BLACK);
    agent.draw(&draw);
    pursuer.draw(&draw);
    let target_pos = calculate_pursue_target(&agent.state);
    draw.ellipse()
        .xy(vec2(target_pos.x, target_pos.y))
        .radius(5.0);
    draw.to_frame(app, &frame).unwrap();
}

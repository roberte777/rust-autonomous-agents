use autonomous_agents::agents::Agent;
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

    Model {
        _window,
        size,
        agents,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    for agent in &mut _model.agents {
        let steer_force = agent.seek(&Position {
            x: _app.mouse.x,
            y: _app.mouse.y,
        });
        agent.apply_force(steer_force);
        agent.move_agent();
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    //draw the current mouse pos as circle on screen
    draw.ellipse()
        .xy(vec2(app.mouse.x, app.mouse.y))
        .radius(10.0);
    for agent in &_model.agents {
        draw.tri()
            .xy(vec2(agent.pos.x as f32, agent.pos.y as f32))
            .w_h(agent.width, agent.height)
            .rotate(agent.rotation)
            .color(STEELBLUE);

        //code to draw the target line if you want to see the direction
        //the triangles are going
        // draw.line()
        //     .start(vec2(agent.pos.x, agent.pos.y))
        //     .end(vec2(
        //         agent.pos.x + agent.desired.x,
        //         agent.pos.y + agent.desired.y,
        //     ))
        //     .color(WHITE);
    }
    draw.to_frame(app, &frame).unwrap();
}

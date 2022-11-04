use super::{AgentState, PhysicsState, Prey};
use std::ops::{Div, Mul, Sub};

use crate::physics::{Position, Vector};
use nannou::prelude::*;

//Steers the agent towards its target
pub fn seek(agent_state: &AgentState, target: &Position) -> Vector {
    //desired vector is a straight line to its target.
    let mut desired = Vector {
        x: target.x - agent_state.pos.x,
        y: target.y - agent_state.pos.y,
    };
    //setting the desired vector to the max max_speed
    //because the agent wants to reach target as fast as possible
    desired.set_mag(agent_state.max_speed);
    //calculation for steering force is desired - current velocity
    let mut steer_vector = desired - agent_state.velocity;
    //make sure force is not bigger than max force
    steer_vector.limit(agent_state.max_force);
    return steer_vector;
}
pub fn flee(agent_state: &AgentState, target: &Position) -> Vector {
    return seek(agent_state, target) * -1.0;
}
pub fn pursue(agent_state: &AgentState, target_agent: &AgentState) -> Vector {
    let mut prediction = target_agent.pos.clone();
    prediction.x = prediction.x + (target_agent.velocity.x * 10.0);
    prediction.y = prediction.y + (target_agent.velocity.y * 10.0);
    return seek(agent_state, &prediction);
}
pub fn calculate_pursue_target(target_agent: &AgentState) -> Position {
    let mut prediction = target_agent.pos.clone();
    prediction.x = prediction.x + (target_agent.velocity.x * 10.0);
    prediction.y = prediction.y + (target_agent.velocity.y * 10.0);
    return prediction;
}
pub fn evade(agent_state: &AgentState, target_agent: &AgentState) -> Vector {
    let mut val = pursue(agent_state, target_agent);
    val.x = val.x * -1.0;
    val.y = val.y * -1.0;
    return val;
}
pub fn alignment(agent_state: &PhysicsState, other_boids: &Vec<&Prey>) -> Vec2 {
    let len = other_boids.len();

    if len == 0 {
        return vec2(0.0, 0.0);
    }

    let mut average_velocity = vec2(0.0, 0.0);
    for i in other_boids {
        average_velocity += i.state.velocity;
    }
    average_velocity = average_velocity.div(len as f32);
    average_velocity.sub(agent_state.velocity) / 2.5
}

pub fn cohesion(agent_state: &PhysicsState, other_boids: &Vec<&Prey>) -> Vec2 {
    let len = other_boids.len();

    if len == 0 {
        return vec2(0.0, 0.0);
    }

    let mut average_position = vec2(0.0, 0.0);
    for i in other_boids {
        average_position += i.state.pos;
    }
    average_position = average_position.div(len as f32);
    average_position.sub(agent_state.pos) / 20.0
}

pub fn separation(agent_state: &PhysicsState, other_boids: &Vec<&Prey>) -> Vec2 {
    let len = other_boids.len();

    if len == 0 {
        return vec2(0.0, 0.0);
    }

    let mut average_seperation = vec2(0.0, 0.0);
    for i in other_boids {
        let difference_vec = i
            .state
            .pos
            .sub(agent_state.pos)
            .div(agent_state.pos.distance(i.state.pos) * 2.0);
        average_seperation -= difference_vec;
    }
    average_seperation * 2.0
}

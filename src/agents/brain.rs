use super::{AgentState, Prey};
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
// pub fn alignment(agent_state: &AgentState, neighbors: &Vec<&Prey>) -> Vector {
//     let len = neighbors.len();

//     if len == 0 {
//         return Vector { x: 0.0, y: 0.0 };
//     }

//     let mut average_velocity = Vector { x: 0.0, y: 0.0 };
//     for i in neighbors {
//         average_velocity = average_velocity + i.state.velocity;
//     }
//     // need to implement possiblity to divide vector by magnitude
//     average_velocity = average_velocity / (len as f32);
//     return (average_velocity - agent_state.velocity) / 2.5;
// }

// pub fn cohesion(agent_state: &AgentState, neighbors: &Vec<&Prey>) -> Vector {
//     let len = neighbors.len();

//     if len == 0 {
//         return Vector { x: 0.0, y: 0.0 };
//     }

//     let mut average_position = Position { x: 0.0, y: 0.0 };
//     for neighbor in neighbors {
//         average_position = average_position + neighbor.state.pos;
//     }
//     average_position = average_position / (len as f32);
//     average_position = (average_position - agent_state.pos) / 20.0;
//     return Vector {
//         x: average_position.x,
//         y: average_position.y,
//     };
// }

// pub fn separation(agent_state: &AgentState, neighbors: &Vec<&Prey>) -> Vector {
//     let len = neighbors.len();

//     if len == 0 {
//         return Vector { x: 0.0, y: 0.0 };
//     }

//     let mut average_seperation = Vector { x: 0.0, y: 0.0 };
//     for neighbor in neighbors {
//         let mut difference_vec = Vector {
//             x: neighbor.state.pos.x - agent_state.pos.x,
//             y: neighbor.state.pos.y - agent_state.pos.y,
//         };
//         difference_vec = difference_vec / (agent_state.pos.distance(&neighbor.state.pos) * 2.0);
//         average_seperation = average_seperation - difference_vec;
//     }
//     average_seperation * 1.5
// }

pub fn alignment(agent_state: &AgentState, other_boids: &Vec<&Prey>) -> Vector {
    let len = other_boids.len();

    if len == 0 {
        return Vector { x: 0.0, y: 0.0 };
    }

    let mut average_velocity = vec2(0.0, 0.0);
    for i in other_boids {
        let t = vec2(i.state.velocity.x, i.state.velocity.y);
        average_velocity += t;
    }
    average_velocity = average_velocity.div(len as f32);
    let temp = average_velocity.sub(vec2(agent_state.velocity.x, agent_state.velocity.y)) / 2.5;
    return Vector {
        x: temp.x,
        y: temp.y,
    };
}

pub fn cohesion(agent_state: &AgentState, other_boids: &Vec<&Prey>) -> Vector {
    let len = other_boids.len();

    if len == 0 {
        return Vector { x: 0.0, y: 0.0 };
    }

    let mut average_position = vec2(0.0, 0.0);
    for i in other_boids {
        let t = vec2(i.state.pos.x, i.state.pos.y);
        average_position += t;
    }
    average_position = average_position.div(len as f32);
    let temp = average_position.sub(vec2(agent_state.pos.x, agent_state.pos.y)) / 20.0;
    return Vector {
        x: temp.x,
        y: temp.y,
    };
}

pub fn separation(agent_state: &AgentState, other_boids: &Vec<&Prey>) -> Vector {
    let len = other_boids.len();

    if len == 0 {
        return Vector { x: 0.0, y: 0.0 };
    }

    let mut average_seperation = vec2(0.0, 0.0);
    for i in other_boids {
        let t = vec2(i.state.pos.x, i.state.pos.y);
        let difference_vec = t
            .sub(vec2(agent_state.pos.x, agent_state.pos.y))
            .div(vec2(agent_state.pos.x, agent_state.pos.y).distance(t) * 2.0);
        average_seperation -= difference_vec;
    }
    let temp = average_seperation * 1.5;
    return Vector {
        x: temp.x,
        y: temp.y,
    };
}

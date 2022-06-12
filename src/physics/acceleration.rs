use nannou::prelude::Vec2;
use std::ops::Mul;

pub fn set_max_acc(max_acc: f32, acc: &Vec2) -> Vec2 {
    let acc_len = acc.length_squared();

    let mut new_acc = acc.clone();

    if acc_len > max_acc * max_acc {
        new_acc = acc.normalize_or_zero();
        new_acc = new_acc.mul(max_acc);
    }
    new_acc
}

use cgmath::{BaseNum, MetricSpace};
use simulation::*;
use crate::agents;

pub fn closest(me: Position, others: Vec<&mut agents::TagAgent>) -> &mut agents::TagAgent {
    others.sort_by(|a, b| {
        return me.distance(a.position).partial_cmp(&me.distance(b.position)).unwrap();
    });
    return others[0];
}

pub fn clamp<T: BaseNum>(value: T, min: T, max: T) -> T {
    let result = value;
    if value < min {
        result = min;
    }
    if value > max {
        result = max;
    }
    return value;
}

pub fn clamp_position(value: &mut Position, min: f32, max: f32) {
    value.x = clamp(value.x, min, max);
    value.y = clamp(value.y, min, max);
}
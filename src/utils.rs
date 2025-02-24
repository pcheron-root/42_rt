use crate::constants::EPSILON;
use std::ops::{Sub, Add, Mul};

pub fn are_almost_equal(x: f32, y: f32) -> bool {
    (x - y).abs() < EPSILON
}

pub fn lerp<T>(start: T, end: T, t: f32) -> T
where
    T: Copy + Sub<Output = T> + Add<Output = T> + Mul<f32, Output = T>,
{
    let t = t.clamp(0., 1.);

    start * (1. - t) + end * t
}

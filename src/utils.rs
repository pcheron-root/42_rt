use crate::constants::EPSILON;
use std::ops::{Add, Mul, Sub};

pub fn are_almost_equal(x: f64, y: f64) -> bool {
    (x - y).abs() < EPSILON
}

pub fn lerp<T>(start: T, end: T, t: f64) -> T
where
    T: Sub<Output = T> + Add<Output = T> + Mul<f64, Output = T>,
{
    let t = t.clamp(0., 1.);

    start * (1. - t) + end * t
}

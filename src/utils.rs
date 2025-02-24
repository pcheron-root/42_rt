use crate::constants::EPSILON;

pub fn are_almost_equal(x: f32, y: f32) -> bool {
    (x - y).abs() < EPSILON
}
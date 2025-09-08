
use crate::EPSILON;

pub fn are_almost_equal(x: f64, y: f64) -> bool {
    (x - y).abs() < EPSILON
}

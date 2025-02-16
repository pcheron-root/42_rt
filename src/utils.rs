
pub fn are_almost_equal(x: f32, y: f32) -> bool {
    let epsilon = 0.00001;
    (x - y).abs() < epsilon
}


// use crate::{World, Point};

pub fn is_shadowed(world: &World, point: &Point) -> bool {
    let epsilon = 0.00001;
    (x - y).abs() < epsilon
}

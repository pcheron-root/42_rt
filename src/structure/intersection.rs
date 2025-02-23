use crate::{Point, Vector};

#[derive(Debug, Clone)]
pub struct Intersection {
    pub point: Point,
    pub normal: Vector,
    pub t: f32,
}
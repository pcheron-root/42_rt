use crate::{Point, Vector};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: f32) -> Point {
        self.origin.clone() + self.direction.clone() * t
    }
}

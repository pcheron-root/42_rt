use crate::{Intersect, LocalIntersection, Point, Ray, Vector};

#[derive(Debug, Clone)]
pub struct Torus {
    pub major_radius: f32, // Distance from center of tube to center of torus
    pub minor_radius: f32, // Radius of the tube
}

impl Torus {
    pub fn new(major_radius: f32, minor_radius: f32) -> Self {
        Self {
            major_radius,
            minor_radius,
        }
    }
}

impl Intersect for Torus {
    fn intersect(&self, _: Ray) -> Option<LocalIntersection> {
        None
    }

    fn normal_at(&self, _: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

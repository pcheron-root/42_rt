use crate::Intersect;
use crate::LocalIntersection;
use crate::Plane;
use crate::Ray;
use crate::Sphere;
use crate::Cube;

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube)
}

impl Shape {
    pub fn intersect(&self, ray: Ray) -> Option<LocalIntersection> {
        match self {
            Shape::Sphere(s) => s.intersect(ray),
            Shape::Plane(s) => s.intersect(ray),
            Shape::Cube(s) => s.intersect(ray),
        }
    }
}

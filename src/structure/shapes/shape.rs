use crate::Intersect;
use crate::LocalHit;
use crate::Plane;
use crate::Ray;
use crate::Sphere;

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

impl Shape {
    pub fn intersect(&self, ray: Ray) -> Option<LocalHit> {
        match self {
            Shape::Sphere(s) => s.intersect(ray),
            Shape::Plane(s) => s.intersect(ray),
        }
    }
}

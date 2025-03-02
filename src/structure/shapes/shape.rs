use crate::Intersect;
use crate::LocalHit;
use crate::Ray;
use crate::Sphere;

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn intersect(&self, ray: Ray) -> Option<LocalHit> {
        match self {
            Shape::Sphere(s) => s.intersect(ray),
        }
    }
}

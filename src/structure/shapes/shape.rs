use crate::Ray;
use crate::Sphere;
use crate::LocalHit;
use crate::Intersect;

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Option<LocalHit> {
        match self {
            Shape::Sphere(s) => s.intersect(ray),
        }
    }
}
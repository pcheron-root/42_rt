use crate::Intersection;
use crate::Ray;

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

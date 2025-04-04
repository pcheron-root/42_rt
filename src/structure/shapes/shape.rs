use crate::Cone;
use crate::Cube;
use crate::Cylinder;
use crate::Disk;
use crate::Intersect;
use crate::LocalIntersection;
use crate::Plane;
use crate::Ray;
use crate::Sphere;
use crate::Triangle;
use crate::Tube;

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),
    Triangle(Triangle),
    Tube(Tube),
    Disk(Disk),
}

impl Shape {
    pub fn intersect(&self, ray: Ray) -> Option<LocalIntersection> {
        match self {
            Shape::Sphere(s) => s.intersect(ray),
            Shape::Plane(s) => s.intersect(ray),
            Shape::Cube(s) => s.intersect(ray),
            Shape::Cylinder(s) => s.intersect(ray),
            Shape::Cone(s) => s.intersect(ray),
            Shape::Triangle(s) => s.intersect(ray),
            Shape::Tube(s) => s.intersect(ray),
            Shape::Disk(s) => s.intersect(ray),
        }
    }
}

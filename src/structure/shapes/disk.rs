use crate::constants::EPSILON;
use crate::{Intersect, LocalIntersection, Point, Ray, Vector};

#[derive(Debug, Clone)]
pub struct Disk {
    pub radius: f32,
}

impl Disk {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Intersect for Disk {
    fn intersect(&self, ray: Ray) -> Option<LocalIntersection> {
        if ray.direction.y.abs() < EPSILON {
            return None;
        }

        let t = -ray.origin.y / ray.direction.y;

        if t < 0.0 {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        if (x.powi(2) + z.powi(2)) > self.radius.powi(2) {
            return None;
        }

        let point = ray.position(t);
        let normal = if ray.origin.y > 0.0 {
            self.normal_at(point)
        } else {
            -self.normal_at(point)
        };

        Some(LocalIntersection { point, normal, t })
    }

    fn normal_at(&self, _point: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

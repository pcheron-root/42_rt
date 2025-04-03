use crate::{constants::EPSILON, Intersect, LocalIntersection, Point, Ray, Vector};

#[derive(Debug, Clone)]
pub struct Cylinder {
    radius: f32,
    height: f32,
}

impl Cylinder {
    pub fn new(radius: f32, height: f32) -> Self {
        Self { radius, height }
    }
}

impl Intersect for Cylinder {
    fn intersect(&self, ray: Ray) -> Option<LocalIntersection> {
        let a = (ray.direction.x).powf(2.0) + (ray.direction.z).powf(2.0);

        if a < EPSILON {
            return None;
        }

        let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
        let c = (ray.origin.x).powf(2.0) + (ray.origin.z).powf(2.0) - self.radius.powf(2.0);
        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        let t = if t0 < 0.0 { t1 } else { t0 };
        if t < 0.0 {
            return None;
        }

        let point = ray.position(t);
        let normal = self.normal_at(point);

        let hh = self.height / 2.0;
        if point.y >= -hh && point.y <= hh {
            Some(LocalIntersection { point, normal, t })
        } else {
            None
        }
    }

    fn normal_at(&self, point: Point) -> Vector {
        Vector::new(point.x, 0.0, point.z).normalize()
    }
}

use crate::constants::EPSILON;
use crate::{Intersect, LocalIntersection, Point, Ray, Vector};

#[derive(Debug, Clone)]
pub struct Tube {
    pub height: f64,
    pub radius: f64,
}

impl Tube {
    pub fn new(radius: f64, height: f64) -> Self {
        Self { radius, height }
    }
}

impl Intersect for Tube {
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

        let mut ts = vec![t0, t1];
        ts.sort_by(|u, v| u.partial_cmp(v).unwrap());

        for t in ts {
            if t < 0.0 {
                continue;
            }

            let point = ray.position(t);
            let hh = self.height / 2.0;
            if point.y < -hh || point.y > hh {
                continue;
            }

            let normal = if t == t0 {
                self.normal_at(point)
            } else {
                -self.normal_at(point)
            };

            return Some(LocalIntersection { point, normal, t });
        }

        None
    }

    fn normal_at(&self, point: Point) -> Vector {
        Vector::new(point.x, 0.0, point.z).normalize()
    }
}

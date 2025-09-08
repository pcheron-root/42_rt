use crate::{EPSILON, Intersect, LocalIntersection, Point, Ray, Vector};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    normal: Vector,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        let edge1 = p2 - p1;
        let edge2 = p3 - p1;
        let normal = edge1.cross(&edge2).normalize();

        Self { p1, p2, p3, normal }
    }
}

impl Intersect for Triangle {
    fn intersect(&self, ray: Ray) -> Option<LocalIntersection> {
        let edge1 = self.p2 - self.p1;
        let edge2 = self.p3 - self.p1;

        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);

        if a.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.p1;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);

        if t <= EPSILON {
            return None;
        }

        let point = ray.position(t);
        let normal = if ray.direction.dot(&self.normal) > 0.0 {
            -self.normal
        } else {
            self.normal
        };

        Some(LocalIntersection { point, normal, t })
    }

    fn normal_at(&self, _: Point) -> Vector {
        self.normal
    }
}

use crate::EPSILON;
use crate::Intersect;
use crate::LocalIntersection;
use crate::Point;
use crate::Ray;
use crate::Vector;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Plane {}

impl Plane {
    pub fn new() -> Self {
        Plane {}
    }
}

impl Intersect for Plane {
    fn intersect(&self, ray: Ray) -> Option<LocalIntersection> {
        if (ray.direction.y).abs() < EPSILON {
            return None;
        }

        let t = -ray.origin.y / ray.direction.y;
        if t < 0. {
            return None;
        }
        let point = ray.position(t);

        let normal = if ray.origin.y > 0.0 {
            self.normal_at(point)
        } else {
            -self.normal_at(point)
        };

        Some(LocalIntersection { t, point, normal })
    }

    fn normal_at(&self, _point: Point) -> Vector {
        Vector::new(0., 1., 0.)
    }
}

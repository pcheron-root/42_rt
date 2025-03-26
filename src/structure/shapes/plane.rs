use crate::constants::EPSILON;
use crate::Intersect;
use crate::LocalHit;
use crate::Point;
use crate::Ray;
use crate::Vector;

#[derive(Debug, Clone)]
pub struct Plane {}

impl Plane {
    pub fn new() -> Plane {
        Plane {}
    }
}

impl Intersect for Plane {
    fn intersect(&self, ray: Ray) -> Option<LocalHit> {
        if (ray.direction.y).abs() < EPSILON {
            return None;
        }

        let t = -ray.origin.y / ray.direction.y;
        if t < 0. {
            return None;
        }
        let point = ray.position(t);
        let mut normal = self.normal_at(point);
        if ray.origin.y < 0.0 {
            normal = normal * -1.;
        }

        Some(LocalHit { t, point, normal })
    }

    fn normal_at(&self, _point: Point) -> Vector {
        Vector::new(0., 1., 0.)
    }
}

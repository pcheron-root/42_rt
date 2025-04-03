use std::mem::swap;

use crate::{constants::EPSILON, Intersect, Point, Vector, LocalIntersection, Ray};

#[derive(Debug, Clone)]
pub struct Cube {
    pub size: f32,
}

impl Cube {
    pub fn new(size: f32) -> Self {
        Self { size }
    }
}

impl Intersect for Cube {
    fn intersect(&self, ray: Ray) -> Option<LocalIntersection> {
        let check_axis = |o: f32, d: f32| -> (f32, f32) {
            let hs = self.size / 2.0;
            let tmin_num = -hs - o;
            let tmax_num =  hs - o;

            let (mut tmin, mut tmax) =
                if d.abs() >= EPSILON {
                    (tmin_num / d, tmax_num / d)
                } else {
                    (tmin_num * f32::INFINITY, tmax_num * f32::INFINITY)
                };

            if tmin > tmax { swap(&mut tmin, &mut tmax); }

            (tmin, tmax)
        };

        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

        let tmin = xtmin.max(ytmin.max(ztmin));
        let tmax = xtmax.min(ytmax.min(ztmax));

        if tmin > tmax {
            return None;
        }

        if tmax < 0.0 {
            return None;
        }

        let t = if tmin < 0.0 { tmax } else { tmin };
        let point = ray.position(t);
        let normal = self.normal_at(point);

        Some(LocalIntersection {
            point,
            normal,
            t
        })
    }

    fn normal_at(&self, point: Point) -> Vector {
        let max = (point.x).abs().max((point.y).abs().max((point.z).abs()));

        if max == (point.x).abs() {
            Vector::new(point.x, 0.0, 0.0)
        } else if max == (point.y).abs() {
            Vector::new(0.0, point.y, 0.0)
        } else {
            Vector::new(0.0, 0.0, point.z)
        }
    }
}
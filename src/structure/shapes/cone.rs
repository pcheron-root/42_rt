use crate::constants::EPSILON;

use crate::{Intersect, LocalIntersection, Point, Ray, Vector};

#[derive(Debug, Clone)]
pub struct Cone {
    pub radius: f64,
    pub height: f64,
}

impl Cone {
    pub fn new(radius: f64, height: f64) -> Self {
        Self { radius, height }
    }
}

impl Intersect for Cone {
    fn intersect(&self, ray: Ray) -> Option<LocalIntersection> {
        let intersect_side = || -> Option<f64> {
            let tan_theta = self.radius / self.height;
            let tan_theta_squared = tan_theta.powf(2.0);

            let a = (ray.direction.x).powf(2.0) + (ray.direction.z).powf(2.0)
                - tan_theta_squared * (ray.direction.y).powf(2.0);
            let b = 2.0
                * (ray.origin.x * ray.direction.x + ray.origin.z * ray.direction.z
                    - tan_theta_squared * ray.origin.y * ray.direction.y
                    + tan_theta_squared * self.height * ray.direction.y);
            let c = (ray.origin.x).powf(2.0) + (ray.origin.z).powf(2.0)
                - tan_theta_squared
                    * ((ray.origin.y).powf(2.0) - 2.0 * self.height * ray.origin.y
                        + self.height.powf(2.0));

            let discriminant = b.powf(2.0) - 4.0 * a * c;

            if discriminant.abs() < EPSILON {
                let t = -b / (2.0 * a);
                let p = ray.position(t);

                if t > EPSILON && p.y >= 0.0 && p.y <= self.height {
                    Some(t)
                } else {
                    None
                }
            } else if discriminant > 0.0 {
                let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
                let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

                let p1 = ray.position(t0);
                let p2 = ray.position(t1);

                let valid_t0 = t0 > EPSILON && p1.y >= 0.0 && p1.y <= self.height;
                let valid_t1 = t1 > EPSILON && p2.y >= 0.0 && p2.y <= self.height;

                if valid_t0 && valid_t1 {
                    Some(t0.min(t1))
                } else if valid_t0 {
                    Some(t0)
                } else if valid_t1 {
                    Some(t1)
                } else {
                    None
                }
            } else {
                None
            }
        };

        let intersect_base = || -> Option<f64> {
            if ray.direction.y.abs() > EPSILON {
                let t = -ray.origin.y / ray.direction.y;
                let p = ray.position(t);

                let distance_squared = (p.x).powf(2.0) + (p.z).powf(2.0);
                if t > EPSILON && distance_squared <= self.radius.powf(2.0) {
                    Some(t)
                } else {
                    None
                }
            } else {
                None
            }
        };

        let t_side = intersect_side();
        let t_base = intersect_base();

        match (t_side, t_base) {
            (Some(t_s), Some(t_b)) => {
                let t = t_s.min(t_b);
                let point = ray.position(t);
                let normal = self.normal_at(point);
                Some(LocalIntersection { point, normal, t })
            }
            (Some(t_s), None) => {
                let point = ray.position(t_s);
                let normal = self.normal_at(point);
                Some(LocalIntersection {
                    point,
                    normal,
                    t: t_s,
                })
            }
            (None, Some(t_b)) => {
                let point = ray.position(t_b);
                let normal = self.normal_at(point);
                Some(LocalIntersection {
                    point,
                    normal,
                    t: t_b,
                })
            }
            (None, None) => None,
        }
    }

    fn normal_at(&self, point: Point) -> Vector {
        if point.y.abs() < EPSILON {
            return Vector::new(0.0, -1.0, 0.0);
        }

        let tan_theta = self.radius / self.height;

        let axis_to_point = Vector::new(point.x, 0.0, point.z);
        let distance_from_axis = axis_to_point.magnitude();

        if distance_from_axis < EPSILON {
            return Vector::new(0.0, 1.0, 0.0);
        }

        Vector::new(
            point.x / distance_from_axis,
            tan_theta,
            point.z / distance_from_axis,
        )
        .normalize()
    }
}

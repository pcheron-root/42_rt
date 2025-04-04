use crate::{constants::EPSILON, Intersect, LocalIntersection, Point, Ray, Vector};

#[derive(Debug, Clone)]
pub struct Cylinder {
    pub radius: f32,
    pub height: f32,
}

impl Cylinder {
    pub fn new(radius: f32, height: f32) -> Self {
        Self { radius, height }
    }
}

impl Intersect for Cylinder {
    fn intersect(&self, ray: Ray) -> Option<LocalIntersection> {
        let intersect_tube = || -> Option<f32> {
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
            let hh = self.height / 2.0;

            if point.y >= -hh && point.y <= hh {
                Some(t)
            } else {
                None
            }
        };

        let check_cap = |ray: Ray, t: f32| -> bool {
            if t < 0.0 {
                return false;
            }

            let x = ray.origin.x + t * ray.direction.x;
            let z = ray.origin.z + t * ray.direction.z;

            (x.powf(2.0) + z.powf(2.0)) <= self.radius.powf(2.0)
        };

        let intersect_caps = || -> Option<f32> {
            if ray.direction.y.abs() < EPSILON {
                return None;
            }

            let hh = self.height / 2.0;
            let mut t0 = None;
            let mut t1 = None;

            let t = (-hh - ray.origin.y) / ray.direction.y;
            if check_cap(ray, t) {
                t0 = Some(t);
            }

            let t = (hh - ray.origin.y) / ray.direction.y;
            if check_cap(ray, t) {
                t1 = Some(t);
            }

            let t = if t0.is_some() && t1.is_some() {
                let t0 = t0.unwrap();
                let t1 = t1.unwrap();

                f32::min(t0, t1)
            } else if t0.is_some() {
                t0.unwrap()
            } else if t1.is_some() {
                t1.unwrap()
            } else {
                return None;
            };

            if t < EPSILON {
                return None;
            }

            Some(t)
        };

        let t_tube = intersect_tube();
        let t_caps = intersect_caps();

        match (t_tube, t_caps) {
            (Some(t_t), Some(t_c)) => {
                let t = t_t.min(t_c);
                let point = ray.position(t);
                let normal = self.normal_at(point);
                Some(LocalIntersection { point, normal, t })
            }
            (Some(t_t), None) => {
                let point = ray.position(t_t);
                let normal = self.normal_at(point);
                Some(LocalIntersection {
                    point,
                    normal,
                    t: t_t,
                })
            }
            (None, Some(t_c)) => {
                let point = ray.position(t_c);
                let normal = self.normal_at(point);
                Some(LocalIntersection {
                    point,
                    normal,
                    t: t_c,
                })
            }
            (None, None) => None,
        }
    }

    fn normal_at(&self, point: Point) -> Vector {
        let hh = self.height / 2.0;
        let distance = (point.x).powf(2.0) + (point.z).powf(2.0);

        if distance < (self.radius).powf(2.0) && point.y >= hh - EPSILON {
            Vector::new(0.0, 1.0, 0.0)
        } else if distance < (self.radius).powf(2.0) && point.y <= -hh + EPSILON {
            Vector::new(0.0, -1.0, 0.0)
        } else {
            Vector::new(point.x, 0.0, point.z).normalize()
        }
    }
}

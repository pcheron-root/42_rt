use crate::Intersect;
use crate::LocalIntersection;
use crate::Point;
use crate::Ray;
use crate::Vector;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Sphere { radius }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: Ray) -> Option<LocalIntersection> {
        let center = Point::new(0., 0., 0.);

        let o = ray.origin - center;
        let d = ray.direction;
        let r = self.radius;

        let a = d.dot(&d);
        let b = 2.0 * o.dot(&d);
        let c = o.dot(&o) - r * r;

        let discriminant: f32 = b * b - 4.0 * a * c;

        // No intersection if discriminant is negative
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let t0 = (-b - sqrt_d) / (2.0 * a);
        let t1 = (-b + sqrt_d) / (2.0 * a);

        // Both intersections are behind the ray origin
        if t1 < 0.0 {
            return None;
        }

        // Choose the closest valid intersection
        let t = if t0 < 0.0 { t1 } else { t0 };

        // Calculate intersection point and normal
        let point = ray.position(t);
        let normal = self.normal_at(point);

        Some(LocalIntersection { point, normal, t })
    }

    fn normal_at(&self, point: Point) -> Vector {
        Vector::new(point.x, point.y, point.z) / self.radius
    }
}

use crate::Intersection;
use crate::Point;
use crate::Ray;
use crate::Shape;
use crate::Vector;

pub struct Sphere {
    pub radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Sphere {
        Sphere { radius }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let center = Point::new([0., 0., 0.]);

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
        let point = ray.origin + d * t;
        let normal = Vector::new([point.data.x, point.data.y, point.data.z]) / r;

        Some(Intersection { point, normal, t })
    }
}

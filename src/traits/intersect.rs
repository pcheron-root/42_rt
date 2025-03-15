use crate::structure::intersection::LocalIntersection;
use crate::{Point, Ray, Vector};

pub trait Intersect {
    fn intersect(&self, ray: Ray) -> Option<LocalIntersection>;
    fn normal_at(&self, point: Point) -> Vector;
}

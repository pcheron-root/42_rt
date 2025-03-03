use crate::structure::intersection::LocalHit;
use crate::{Point, Ray, Vector};

pub trait Intersect {
    fn intersect(&self, ray: Ray) -> Option<LocalHit>;
    fn normal_at(&self, point: Point) -> Vector;
}

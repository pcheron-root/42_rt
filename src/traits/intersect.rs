use std::fmt::Debug;
use crate::{Point, Ray, Vector, LocalIntersection};

pub trait Intersect: Debug {
    fn intersect(&self, ray: Ray) -> Option<LocalIntersection>;
    fn normal_at(&self, point: Point) -> Vector;
}

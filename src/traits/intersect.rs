use crate::structure::intersection::LocalHit;
use crate::Ray;

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<LocalHit>;
}

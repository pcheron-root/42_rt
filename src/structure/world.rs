use crate::{Intersection, Object, Ray};

pub struct World {
    objects: Vec<Object>,
    // pub lights: Vec<Light>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new()
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let mut closest_intersection = None;
        
        for object in &self.objects {
            let intersection = object.intersect(ray);

            if intersection.is_some() {
                if closest_intersection.is_none() {
                    closest_intersection = intersection
                } else {
                    if closest_intersection.clone().unwrap().t > intersection.clone().unwrap().t {
                        closest_intersection = intersection;
                    }
                }
            }
        }

        closest_intersection
    }
}
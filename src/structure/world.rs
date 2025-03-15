use crate::{Color, Intersection, Light, Object, Point, Ray, Canvas};

pub struct World {
    pub objects: Vec<Object>,
    pub light: Light,
}

impl World {
    pub fn new() -> World {
        World {
            light: Light::new(Point::new(100., 100., 0.), Color::new(1., 1., 1.)),
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light) {
        self.light = light;
    }

    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let mut closest_intersection = None;

        for object in &self.objects {
            let intersection = object.intersect(ray.clone());

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

    pub fn is_shadowed(&self, point: &Point) -> bool {
        let v = self.light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalize();
    
        let r = Ray::new(*point, direction);
    
        let intersection = self.intersect(r);
    
        if intersection.is_some() {
            let h = intersection.unwrap();
            if h.t < distance {
                return true;
            }
        }
    
        false
    }
    
    pub fn shade_it(&self, comps: &Intersection) -> Color {
        let shadowed = self.is_shadowed(&comps.over_point);
    
        Canvas::lighting_ext(
            &comps.object.material,
            &self.light,
            &comps.point,
            &comps.hit_normal,
            &comps.normal,
            shadowed,
        )
    }
}

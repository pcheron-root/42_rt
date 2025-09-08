
use serde::{Deserialize, Serialize};

use crate::{Object, Light, Ray, Intersection, Point, Vector, Color, TextureType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new() -> World {
        World {
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }
    
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn intersect(&self, ray: &Ray, n1: f64) -> Option<Intersection> {
        let mut closest_intersection = None;

        for object in &self.objects {
            let intersection = object.intersect(ray, n1);
            
            if intersection.is_some() {
                // eprintln!("Je tombe sur l'obj {:?} pos: {:?}", object.material.color, object.position);
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

    // shadow and light
    pub fn lighting(
        obj: &Object,
        light: &Light,
        point: &Point,
        eyev: &Vector,
        normalv: &Vector,
        shadowed: bool,
    ) -> Color {
        let effective_color = if let Some(pattern) = &obj.material.pattern {
             pattern.stripe_at_object(obj, point)
        }
        else if let Some(texs) = &obj.material.tex {
            let mut color = obj.material.color * light.intensity;
            for tex in texs {
                if tex.kind == TextureType::Diffuse {
                    color = tex.stripe_at_object(obj, point);
                }
                else if tex.kind == TextureType::Normal  {
                    color += tex.stripe_at_object(obj, point) * 0.05;
                }
                else if tex.kind == TextureType::Detail {
                    color += tex.stripe_at_object(obj, point) * 0.4;
                }

            }
            color
        }
        else {
            obj.material.color * light.intensity
        };

        let lightv = (light.position - *point).normalize();

        let mut ambient = effective_color * obj.material.ambient;
        let light_dot_normal = lightv.dot(normalv);

        if light_dot_normal < 0. || shadowed == true {
            if let Some(texs) = &obj.material.tex {
                for t in texs {
                    if t.kind == TextureType::Emission {
                        return (t.stripe_at_object(obj, point) + ambient) * 0.5;
                    }
                }
                
            }
            return ambient;
            // teture update -> chercher une teture ambiant dans l'obj 
        }

        let diffuse = effective_color * obj.material.diffuse * light_dot_normal;

        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);

        if reflect_dot_eye <= 0. {
            return ambient + diffuse;
        } else {
            let factor = reflect_dot_eye.powf(obj.material.shininess);
            // teture update -> chercher une teture specular dans l'obj
            let specular = if let Some(texs) = &obj.material.tex {
                
                let mut coef_spec = obj.material.specular;
                for t in texs {
                    if t.kind == TextureType::Specular {
                        coef_spec = t.stripe_at_object(obj, point).to_luminance() * 0.3;
                    }
                }
                light.intensity * coef_spec * factor
            }
            else {
                light.intensity * obj.material.specular * factor
            };


            return ambient + diffuse + specular;
        }
    }

}

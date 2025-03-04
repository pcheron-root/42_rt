use crate::{Color, Intersection, Point};

pub struct Light {
    pub position: Point,
    pub color: Color,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Point, color: Color) -> Light {
        Light {
            position,
            color,
            intensity: Color::new(1., 1., 1.),
        }
    }

    pub fn compute(&self, hit: &Intersection) -> Color {
        let material = &hit.object.material;
        let effective_color = material.color * self.intensity;
        let lightv = (self.position - hit.point).normalize();

        let ambient = effective_color * material.ambient;
        let light_dot_normal = lightv.dot(&hit.normal);

        if light_dot_normal < 0. {
            return ambient;
        }

        let specular;
        let diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflectv = (-lightv).reflect(&hit.normal);
        let reflect_dot_eye = reflectv.dot(&hit.eye);

        if reflect_dot_eye <= 0. {
            specular = Color::new(0., 0., 0.);
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = self.intensity * material.specular * factor;
        }

        ambient + diffuse + specular
    }
}

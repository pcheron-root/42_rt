use crate::{Color, Point, Solid, Texturable};
use std::default::Default;

#[derive(Debug)]
pub struct Material {
    pub texture: Box< dyn Texturable>,
    pub shininess: f32,  // between 10 and 200
    pub ambient: f32,    // between 0 and 1
    pub diffuse: f32,    // between 0 and 1
    pub specular: f32,   // between 0 and 1
    pub reflective: f32, // between 0 and 1
}

impl Default for Material {
    fn default() -> Self {
        let shininess = 200.0;
        let ambient = 0.1;
        let diffuse = 0.9;
        let specular = 0.9;
        let reflective = 0.0;
        let texture = Box::new(Solid::new(Color::new(1.0, 1.0, 1.0)));

        Self {
            shininess,
            specular,
            ambient,
            diffuse,
            texture,
            reflective,
        }
    }
}

impl Material {
    pub fn new() -> Self {
        Material::default()
    }

    pub fn reflective(mut self, reflective: f32) -> Self {
        self.reflective = reflective.clamp(0.0, 1.0);

        self
    }

    pub fn specular(mut self, specular: f32) -> Self {
        self.specular = specular.clamp(0.0, 1.0);

        self
    }

    pub fn diffuse(mut self, diffuse: f32) -> Self {
        self.diffuse = diffuse.clamp(0.0, 1.0);

        self
    }

    pub fn shininess(mut self, shininess: f32) -> Self {
        self.shininess = shininess.clamp(10.0, 200.0);

        self
    }

    pub fn ambient(mut self, ambient: f32) -> Self {
        self.ambient = ambient.clamp(0.0, 1.0);

        self
    }

    pub fn texture(mut self, texture: Box<dyn Texturable>) -> Self {
        self.texture = texture;

        self
    }

    pub fn color_at(&self, point: &Point) -> Color {
        self.texture.color_at(point)
    }
}

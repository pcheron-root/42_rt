use crate::{Color, Pattern};
use std::default::Default;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Color,
    pub pattern: Option<Pattern>,

    pub shininess: f64,  // between 10 and 200
    pub ambient: f64,    // between 0 and 1
    pub diffuse: f64,    // between 0 and 1
    pub specular: f64,   // between 0 and 1
    pub reflective: f64, // between 0 and 1
    pub refractive_index: f64,
    pub transparency: f64,
}

impl Default for Material {
    fn default() -> Self {
        let shininess = 200.0;
        let color = Color::new(1.0, 1.0, 1.0);
        let ambient = 0.1;
        let diffuse = 0.9;
        let specular = 0.9;
        let reflective = 0.0;
        let pattern = None;
        let refractive_index = 1.0;
        let transparency = 0.;

        Self {
            shininess,
            specular,
            color,
            ambient,
            diffuse,
            pattern,
            reflective,
            refractive_index,
            transparency,
        }
    }
}

impl Material {
    pub fn new() -> Self {
        Material::default()
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;

        self
    }

    pub fn reflective(mut self, reflective: f64) -> Self {
        self.reflective = reflective.clamp(0.0, 1.0);

        self
    }

    pub fn specular(mut self, specular: f64) -> Self {
        self.specular = specular.clamp(0.0, 1.0);

        self
    }

    pub fn diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse.clamp(0.0, 1.0);

        self
    }

    pub fn shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess.clamp(10.0, 200.0);

        self
    }

    pub fn ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient.clamp(0.0, 1.0);

        self
    }

    pub fn pattern(mut self, pattern: Pattern) -> Self {
        self.pattern = Some(pattern);

        self
    }
}

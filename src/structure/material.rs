use crate::{Axis, Color, Pattern};

#[derive(Debug, Clone)]
pub struct Material {
    pub shininess: f32, // between 10 and 200

    pub color: Color,
    pub ambient: f32,  // between 0 and 1
    pub diffuse: f32,  // between 0 and 1
    pub specular: f32, // between 0 and 1

    pub pattern: Option<Pattern>,
}

impl Material {
    pub fn new() -> Self {
        let shininess = 200.;
        let color = Color::new(1., 1., 1.);
        let ambient = 0.1;
        let diffuse = 0.9;
        let specular = 0.9;
        let pattern = Some(Pattern::new(
            Color::new(0.8, 0.8, 0.8),
            Color::new(0.4, 0., 0.4),
            Axis::X,
            false,
        ));

        Self {
            shininess,
            specular,
            color,
            ambient,
            diffuse,
            pattern,
        }
    }
}

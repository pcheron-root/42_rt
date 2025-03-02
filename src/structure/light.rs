
use crate::{Color, Point};

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
            intensity: Color::new([1., 1., 1.]),
        }
    }
}

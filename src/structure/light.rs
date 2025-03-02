
use crate::{Color, Point};

pub struct Light {
    pub position: Point,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Point, intensity: Color) -> Light {
        Light {
            position,
            intensity,
        }
    }
}

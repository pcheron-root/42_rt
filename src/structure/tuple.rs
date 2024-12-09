// use crate::traits::type_trait::TypeTrait;

pub struct Tuple {
    pub data: [f32; 4],
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new(data: [f32; 4]) -> Self {
        Tuple {
            data,
            x: data[0],
            y: data[1],
            z: data[2],
            w: data[3],
        }
    }
}

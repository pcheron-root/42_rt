// use crate::traits::type_trait::TypeTrait;

pub struct Tuple {
    pub data: [f32; 4],
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple {
            data: [x, y, z, w],
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

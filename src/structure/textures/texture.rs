use crate::{Color, Gradient, Point, Solid, Stripe, Texturable};

#[derive(Debug, Clone)]
pub enum Texture {
    Gradient(Gradient),
    Solid(Solid),
    Stripe(Stripe),
}

impl Texture {
    pub fn color_at(&self, point: &Point) -> Color {
        let point = self.transform(point);

        match self {
            Texture::Gradient(t) => t.color_at(&point),
            Texture::Solid(t) => t.color_at(&point),
            Texture::Stripe(t) => t.color_at(&point),
        }
    }

    fn transform(&self, point: &Point) -> Point {
        match self {
            Texture::Gradient(t) => t.world_to_local.clone() * point.clone(),
            Texture::Solid(t) => t.world_to_local.clone() * point.clone(),
            Texture::Stripe(t) => t.world_to_local.clone() * point.clone(),
        }
    }
}

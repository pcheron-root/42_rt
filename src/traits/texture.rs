use std::fmt::Debug;

use crate::{Color, Point};

pub trait Texturable: Debug + Clone {
    fn color_at(&self, point: &Point) -> Color;
}

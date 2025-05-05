use std::fmt::Debug;
use crate::Vector;

pub trait Transformable: Debug {
    fn rotate(&mut self, pitch: f32, yaw: f32, roll: f32);
    fn translate(&mut self, vec: Vector);
    fn scale(&mut self, vec: Vector);

    fn update_transformation(&mut self);
}


use crate::Vector;

// unused from chapter 10
pub trait Transform {
    fn rotate(&mut self, pitch: f32, yaw: f32, roll: f32);
    fn translate(&mut self, vec : Vector);
    fn scale(&mut self, vec : Vector);
}

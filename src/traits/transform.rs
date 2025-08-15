use crate::Vector;

pub trait Transform {
    fn rotate(&mut self, pitch: f64, yaw: f64, roll: f64);
    fn translate(&mut self, vec: Vector);
    fn scale(&mut self, vec: Vector);
}

pub trait Dot<Rhs = Self> {
    fn dot(&self, rhs: Rhs) -> f32;
}

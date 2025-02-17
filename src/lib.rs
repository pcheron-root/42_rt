// pub mod linear_interpolation;
pub mod structure;
pub mod traits;
pub mod constants;

pub use structure::matrix::Matrix;
pub use structure::objects::Intersection;
pub use structure::objects::Object;
pub use structure::point::Point;
pub use structure::ray::Ray;
pub use structure::shapes::sphere::Sphere;
pub use structure::tuple::Tuple;
pub use structure::vector::Vector;

pub use traits::dot::Dot;
pub use traits::shape::Shape;

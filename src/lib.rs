pub mod constants;
pub mod structure;
pub mod traits;
pub mod utils;
pub mod light_utils;

pub use structure::matrix::Matrix;
pub use structure::objects::Intersection;
pub use structure::objects::Object;
pub use structure::point::Point;
pub use structure::ray::Ray;
pub use structure::shapes::sphere::Sphere;
pub use structure::tuple::Tuple;
pub use structure::vector::Vector;
pub use structure::color::Color;
pub use structure::canvas::Canvas;
pub use structure::light::Light;
pub use structure::objects::Material;

pub use traits::shape::Shape;

pub use structure::point::SubPoint;

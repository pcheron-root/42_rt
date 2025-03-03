pub mod constants;
pub mod structure;
pub mod traits;
pub mod utils;

pub use structure::camera::Camera;
pub use structure::canvas::Canvas;
pub use structure::color::Color;
pub use structure::intersection::Intersection;
pub use structure::intersection::LocalHit;
pub use structure::light::Light;
pub use structure::matrix::Matrix;
pub use structure::objects::Material;
pub use structure::objects::Object;
pub use structure::point::Point;
pub use structure::ray::Ray;
pub use structure::renderer::Renderer;
pub use structure::shapes::shape::Shape;
pub use structure::shapes::sphere::Sphere;
pub use structure::shapes::plane::Plane;
pub use structure::vector::Vector;
pub use structure::world::World;

pub use traits::intersect::Intersect;

pub mod constants;
pub mod light_utils;
pub mod structure;
pub mod traits;
pub mod utils;

pub use structure::camera::Camera;
pub use structure::camera::Direction;

pub use structure::canvas::Canvas;
pub use structure::color::Color;
pub use structure::intersection::Intersection;
pub use structure::intersection::LocalIntersection;
pub use structure::light::Light;
pub use structure::material::Material;
pub use structure::matrix::Matrix;
pub use structure::objects::Object;
pub use structure::pattern::Pattern;
pub use structure::point::Point;
pub use structure::ray::Ray;
pub use structure::renderer::Renderer;
pub use structure::shapes::plane::Plane;
pub use structure::shapes::shape::Shape;
pub use structure::shapes::sphere::Sphere;
pub use structure::vector::Vector;
pub use structure::world::World;

pub use traits::intersect::Intersect;

pub use traits::transform::Transform;

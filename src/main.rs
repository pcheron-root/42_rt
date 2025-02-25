use rt::Renderer;
use rt::World;
use rt::Camera;
use rt::Color;
use rt::Matrix;
use rt::Object;
use rt::Point;
use rt::Ray;
use rt::Sphere;
use rt::Vector;
use rt::Canvas;
use rt::Shape;
use rt::SubPoint;

// use std::time::Duration;

use minifb::{Window, WindowOptions};

pub fn draw(canvas: &mut Canvas, world: &World, camera: &Camera) {
    
    let view = Matrix::view(
        camera.position.clone(),
        camera.position.clone() + camera.direction.clone(),
        Vector::new([0., 1., 0.])
    );

    let projection = Matrix::projection(
        camera.fov,
        camera.aspect,
        camera.near,
        camera.far
    );

    let view_proj = projection * view;
    let inv_view_proj = view_proj.inverse().unwrap();

    for y in 0..canvas.height {
        let ndc_y = 1.0 - 2.0 * (y as f32 + 0.5) / canvas.height as f32;
        
        for x in 0..canvas.width {
            let ndc_x = 2.0 * (x as f32 + 0.5) / canvas.width as f32 - 1.0;
            
            let origin = inv_view_proj.clone() * Point::new([ndc_x, ndc_y, -1.0]);
            let target = inv_view_proj.clone() * Point::new([ndc_x, ndc_y, 1.0]);
            
            let direction = (target.sub(origin.clone())).normalize();
            
            let ray = Ray::new(Point::new([origin.data.x, origin.data.y, origin.data.z]), direction);
            
            if world.intersect(ray).is_some() {
                canvas.write(x, y, Color::new([1., 0., 0.]));
            } else {
                canvas.write(x, y, Color::new([0., 0., 0.]));
            }
        }
    }
}

fn main() {

    let size = (300, 300);

    let window = Window::new(
        "RT",
        size.0,
        size.1,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    ).unwrap();

    let canvas = Canvas::new(size.0, size.1);

    let camera = Camera::new(
        Point::new([0., 0., 5.]),
        Vector::new([0., 0., -1.]),
        size.0 as f32 / size.1 as f32,
        45f32.to_radians(),
        0.1,
        100.
    );

    let mut s1 = Object::new(Shape::Sphere(Sphere::new(1.)));
    s1.translate(Vector::new([-1., 0., 0.]));
    
    let mut s2 = Object::new(Shape::Sphere(Sphere::new(1.)));
    s2.translate(Vector::new([1., 0., 0.]));
    
    let mut world = World::new();
    world.add_object(s1);
    world.add_object(s2);

    let mut renderer = Renderer::new(
        window,
        canvas,
        world,
        camera
    );

    renderer.render(draw);
}

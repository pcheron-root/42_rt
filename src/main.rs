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

use std::time::Duration;

use minifb::{Window, WindowOptions, Key};

pub fn render(canvas: &mut Canvas, world: &World, camera: &Camera) {

    let aspect_ratio = canvas.width as f32 / canvas.height as f32;
    
    let view = Matrix::view(
        camera.position,
        camera.position + camera.direction,
        Vector::new([0., 1., 0.])
    );

    let projection = Matrix::projection(
        camera.fov,
        aspect_ratio,
        camera.near,
        camera.far
    );

    let view_proj = projection * view;
    let inv_view_proj = view_proj.inverse().unwrap();

    for y in 0..canvas.height {
        let ndc_y = 1.0 - 2.0 * (y as f32 + 0.5) / canvas.height as f32;
        
        for x in 0..canvas.width {
            let ndc_x = 2.0 * (x as f32 + 0.5) / canvas.width as f32 - 1.0;
            
            let origin = inv_view_proj * Point::new([ndc_x, ndc_y, -1.0]);
            let target = inv_view_proj * Point::new([ndc_x, ndc_y, 1.0]);
            
            let direction = (target.sub(origin)).normalize();
            
            let ray = Ray::new(Point::new([origin.data.x, origin.data.y, origin.data.z]), direction);
            
            if world.intersect(ray).is_some() {
                canvas.write(x, y, Color::new([1., 0., 0.]));
            }
        }
    }
}

fn main() {
    let camera = Camera::new(
        Point::new([0., 0., -5.]),
        Vector::new([0., 0., 1.]),
        45f32.to_radians(),
        0.1,
        100.
    );

    let width = 300;
    let height = 300;
    
    let mut window = Window::new(
        "RT",
        width,
        height,
        WindowOptions::default(),
    ).unwrap();

    let mut canvas = Canvas::new(width, height);

    let mut s1 = Object::new(Shape::Sphere(Sphere::new(1.)));
    s1.translate(Vector::new([-1., 0., 0.]));
    
    let mut s2 = Object::new(Shape::Sphere(Sphere::new(1.)));
    s2.translate(Vector::new([1., 0., 0.]));
    
    let mut world = World::new();
    world.add_object(s1);
    world.add_object(s2);
    
    while window.is_open() && !window.is_key_down(Key::Escape) {

        render(&mut canvas, &world, &camera);

        let buffer = canvas.pixels();

        window.update_with_buffer(
            &buffer,
            width,
            height
        ).unwrap();

        std::thread::sleep(Duration::from_millis(16));
    }
}

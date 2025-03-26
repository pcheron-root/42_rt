use rt::light_utils::shade_it;
use rt::Camera;
use rt::Canvas;
use rt::Color;
use rt::Light;
use rt::Matrix;
use rt::Object;
use rt::Plane;
use rt::Point;
use rt::Ray;
use rt::Renderer;
use rt::Shape;
use rt::Sphere;
use rt::Vector;
use rt::World;

use minifb::{Window, WindowOptions};

pub fn draw(canvas: &mut Canvas, world: &World, camera: &Camera) {
    let sky = Color::new(0., 0., 0.);

    let view = Matrix::view(
        camera.position,
        camera.position + camera.direction(),
        Vector::new(0., 1., 0.),
    );

    let projection = Matrix::projection(camera.fov, camera.aspect, camera.near, camera.far);

    let view_proj = projection * view;
    let inv_view_proj = view_proj.inverse().unwrap();

    for y in 0..canvas.height {
        let ndc_y = 1.0 - 2.0 * ((canvas.height - y) as f32 + 0.5) / canvas.height as f32;

        for x in 0..canvas.width {
            let ndc_x = 2.0 * (x as f32 + 0.5) / canvas.width as f32 - 1.0;

            let origin = inv_view_proj.clone() * Point::new(ndc_x, ndc_y, -1.0);
            let target = inv_view_proj.clone() * Point::new(ndc_x, ndc_y, 1.0);

            let direction = (target - origin).normalize();

            let ray = Ray::new(Point::new(origin.x, origin.y, origin.z), direction);

            let hit = world.intersect(ray);
            if hit.is_some() {
                let hit = hit.unwrap();

                let color = shade_it(&world, &hit);

                canvas.write(x, y, color);
            } else {
                canvas.write(x, y, sky);
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
    )
    .unwrap();

    let canvas = Canvas::new(size.0, size.1);

    let camera = Camera::new(
        Point::new(0., 0., 7.),
        Vector::new(0., 0., -1.),
        size.0 as f32 / size.1 as f32,
        45f32.to_radians(),
        0.1,
        100.,
    );

    let mut sphere1 = Object::new(Shape::Sphere(Sphere::new(1.)));
    sphere1.translate(Vector::new(-1., 3., 0.));

    let mut sphere2 = Object::new(Shape::Sphere(Sphere::new(1.)));
    sphere2.translate(Vector::new(1., 3., 0.));

    let mut sphere3 = Object::new(Shape::Sphere(Sphere::new(3.)));
    sphere3.translate(Vector::new(0., -4., 0.));
    
    let plane = Object::new(Shape::Plane(Plane::new()));

    let mut world = World::new();

    world.add_object(sphere1);
    world.add_object(sphere2);
    world.add_object(sphere3);
    world.add_object(plane);

    let light = Light::new(Point::new(0., 10., 0.), Color::new(1., 1., 1.));
    world.add_light(light);

    let mut renderer = Renderer::new(window, canvas, world, camera);

    renderer.render(draw);
}

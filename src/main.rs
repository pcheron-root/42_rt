use rt::Camera;
use rt::Canvas;
use rt::Color;
use rt::Light;
use rt::Matrix;
use rt::Object;
use rt::Point;
use rt::Ray;
use rt::Renderer;
use rt::Shape;
use rt::Sphere;
use rt::Vector;
use rt::World;

use minifb::{Window, WindowOptions};

pub fn draw(canvas: &mut Canvas, world: &World, camera: &Camera) {
    let sky = Color::new([0., 0., 0.]);

    let view = Matrix::view(
        camera.position.clone(),
        camera.position.clone() + camera.direction.clone(),
        Vector::new([0., 1., 0.]),
    );

    let projection = Matrix::projection(camera.fov, camera.aspect, camera.near, camera.far);

    let view_proj = projection * view;
    let inv_view_proj = view_proj.inverse().unwrap();

    for y in 0..canvas.height {
        let ndc_y = -1.0 + 2.0 * (y as f32 + 0.5) / canvas.height as f32;

        for x in 0..canvas.width {
            let ndc_x = 2.0 * (x as f32 + 0.5) / canvas.width as f32 - 1.0;

            let origin = inv_view_proj.clone() * Point::new([ndc_x, ndc_y, -1.0]);
            let target = inv_view_proj.clone() * Point::new([ndc_x, ndc_y, 1.0]);

            let direction = (target - origin.clone()).normalize();

            let ray = Ray::new(
                Point::new([origin.data.x, origin.data.y, origin.data.z]),
                direction,
            );

            let hit = world.intersect(ray);
            if hit.is_some() {
                let hit = hit.unwrap();
                let color = canvas.lighting(&hit.object.material, &world.light, &hit.point, &(ray.direction * -1.), &hit.normal);

                canvas.write(x, y, color);
            } else {
                canvas.write(x, y, sky);
            }
        }
    }

    // let mut obj = Object::new(Box::new(Sphere::new(1.)));
    // // let scale_v = Vector::new([1., 0.5, 1.]); // transfo
    // // obj.scale(&scale_v);
    // // let red = Color::new([1., 0., 0.]);
    // obj.material.color = Color::new([1., 0.2, 1.]);

    // let light = Light {
    //     position: Point::new([-10., 10.,-10.]),
    //     color: Color::new([1.,1.,1.]),
    //     intensity: Color::new([1., 1., 1.]),
    // };

    // y == 0
    // 100
    // -50 

    // for y in 0..canvas.height {
    //     let world_y = half - pixel_size * y as f32;
    //     for x in 0..canvas.width {
    //         let world_x = -half + pixel_size * x as f32;
    //         let position = Point::new([world_x, world_y, wall_z]);
    //         let r = Ray::new(canvas.camera_origin, (position - canvas.camera_origin).normalize());
        
    //         let result = obj.intersect(&r);
    //         if result.is_some() {
    //             let intersect = result.unwrap();
    //             let color = canvas.lighting(&obj.material, &light, &intersect.point, &(r.direction * -1.), &intersect.normal);
                
    //             canvas.write_pixel(x, y, color);
    //         }
    //     }
    // }
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
        Point::new([0., 0., 5.]),
        Vector::new([0., 0., -1.]),
        size.0 as f32 / size.1 as f32,
        45f32.to_radians(),
        0.1,
        100.,
    );

    let mut s1 = Object::new(Shape::Sphere(Sphere::new(1.)));
    s1.translate(Vector::new([-1., 0., 0.]));

    let mut s2 = Object::new(Shape::Sphere(Sphere::new(1.)));
    s2.translate(Vector::new([1., 0., 0.]));

    let mut world = World::new();
    world.add_object(s1);
    world.add_object(s2);

    let light = Light::new(Point::new([0., 2., 0.]), Color::new([1., 1., 1.]));
    world.add_light(light);

    let mut renderer = Renderer::new(window, canvas, world, camera);

    renderer.render(draw);
}

use rt::{
    light_utils::shade_it, Camera, Canvas, Color, Cone, Disk, Light, Matrix, Object, Point, Ray,
    Renderer, Sphere, Transformable, Triangle, Tube, Vector, World,
};

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
        Point::new(0., 0., 10.),
        Vector::new(0., 0., -1.),
        size.0 as f32 / size.1 as f32,
        45f32.to_radians(),
        0.1,
        100.,
    );

    let mut world = World::new();

    let mut sphere = Object::new(Box::new(Sphere::new(1.0)));
    sphere.translate(Vector::new(2.0, 0.0, 0.0));
    world.add_object(sphere);

    let mut tube = Object::new(Box::new(Tube::new(1.0, 3.0)));
    tube.translate(Vector::new(-2.0, 0.0, -1.5));
    tube.rotate(-90.0f32.to_radians(), 0.0, 0.0);
    world.add_object(tube);

    let mut cone = Object::new(Box::new(Cone::new(1.0, 3.0)));
    cone.rotate(-90.0f32.to_radians(), 0.0, 0.0);
    world.add_object(cone);

    let triangle = Object::new(Box::new(Triangle::new(
        Point::new(4.0, 0.0, 0.0),
        Point::new(3.0, 0.0, 0.0),
        Point::new(3.5, 1.0, 0.0),
    )));
    world.add_object(triangle);

    let mut disk = Object::new(Box::new(Disk::new(1.0)));
    disk.rotate(-90.0f32.to_radians(), 0.0, 0.0);
    disk.translate(Vector::new(-3.0, 0.0, 0.0));

    world.add_object(disk);

    let light = Light::new(Point::new(0., 10., 10.), Color::new(1., 1., 1.));
    world.add_light(light);

    let mut renderer = Renderer::new(window, canvas, world, camera);
    renderer.render(draw);
}

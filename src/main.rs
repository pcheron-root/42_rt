use minifb::{Window, WindowOptions};
use rt::{
    light_utils::get_phong_color, Camera, Canvas, Color, Light, Matrix, Object, Point, Ray,
    Renderer, Shape, Sphere, Torus, Transform, Vector, World,
};

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
        let ndc_y = 1.0 - 2.0 * ((canvas.height - y) as f64 + 0.5) / canvas.height as f64;

        for x in 0..canvas.width {
            let ndc_x = 2.0 * (x as f64 + 0.5) / canvas.width as f64 - 1.0;

            let origin = inv_view_proj.clone() * Point::new(ndc_x, ndc_y, -1.0);
            let target = inv_view_proj.clone() * Point::new(ndc_x, ndc_y, 1.0);

            let direction = (target - origin).normalize();

            let ray = Ray::new(Point::new(origin.x, origin.y, origin.z), direction);

            let hit: Option<rt::Intersection> = world.intersect(ray, 1.);
            if hit.is_some() {
                let color = get_phong_color(&world, hit.unwrap());
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
        Point::new(0., 0., 20.),
        Vector::new(0., 0., -1.),
        size.0 as f64 / size.1 as f64,
        45f64.to_radians(),
        0.1,
        100.,
    );

    let mut world = World::new();

    let mut torus = Object::new(Shape::Torus(Torus::new(2., 1.)));
    torus.translate(Vector::new(-3., 0., 0.));
    let mut sphere = Object::new(Shape::Sphere(Sphere::new(2.)));
    sphere.translate(Vector::new(3., 0., 0.));

    world.add_object(torus);
    world.add_object(sphere);

    let light1 = Light::new(Point::new(0., 10., 0.), Color::new(1., 1., 1.));
    let light2 = Light::new(Point::new(0., -10., 0.), Color::new(1., 1., 1.));
    let light3 = Light::new(Point::new(10., 0., 0.), Color::new(1., 1., 1.));
    let light4 = Light::new(Point::new(-10., 0., 0.), Color::new(1., 1., 1.));
    let light5 = Light::new(Point::new(0., 0., 10.), Color::new(1., 1., 1.));
    let light6 = Light::new(Point::new(0., 0., -10.), Color::new(1., 1., 1.));

    world.add_light(light1);
    world.add_light(light2);
    world.add_light(light3);
    world.add_light(light4);
    world.add_light(light5);
    world.add_light(light6);

    let mut renderer = Renderer::new(window, canvas, world, camera);
    renderer.render(draw);
}

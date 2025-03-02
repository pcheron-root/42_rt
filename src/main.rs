use rt::structure::light::Light;
use rt::Color;
// use rt::structure::canvas::Canvas;
use rt::Object;
use rt::Point;
use rt::Ray;
use rt::Sphere;
use rt::Vector;
use rt::Canvas;

// use rt::SubPoint;

// lighting(material: &Material, light: &Light, point: &Point, eyev: &Vector, normalv: &Vector)

pub fn main_loop(canvas: &mut Canvas) { 

    let wall_z = 10.;

    let pixel_size = canvas.wall / canvas.width as f32;
    let half = canvas.wall / 2.;

    let mut obj = Object::new(Box::new(Sphere::new(1.)));
    // let scale_v = Vector::new([1., 0.5, 1.]); // transfo
    // obj.scale(&scale_v);
    // let red = Color::new([1., 0., 0.]);
    obj.material.color = Color::new([1., 0.2, 1.]);

    let light = Light {
        position: Point::new([-10., 10.,-10.]),
        intensity: Color::new([1., 1., 1.]),
    };

    // y == 0
    // 100
    // -50 

    for y in 0..canvas.height {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas.width {
            let world_x = -half + pixel_size * x as f32;
            let position = Point::new([world_x, world_y, wall_z]);
            let r = Ray::new(canvas.camera_origin, (position - canvas.camera_origin).normalize());
        
            let result = obj.intersect(&r);
            if result.is_some() {
                let intersect = result.unwrap();
                let color = canvas.lighting(&obj.material, &light, &intersect.point, &(r.direction * -1.), &intersect.normal);
                
                canvas.write_pixel(x, y, color);
            }
            
        }
    }

}

fn main() {

    let camera_origin = Point::new([0., 0., -5.]);
    let mut _canvas = Canvas::new(500, 500, camera_origin, 7.);
    _canvas.canvas_loop(main_loop, 16);
    
    let obj = Object::new(Box::new(Sphere::new(1.)));

    let ray = Ray::new(Point::new([0., 0., 5.]), Vector::new([0., 0., 1.]));

    let result = obj.intersect(&ray);

    match result {
        Some(_) => println!("Intersection"),
        None => println!("No intersection"),
    }
}

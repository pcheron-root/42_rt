use rt::Color;
// use rt::structure::canvas::Canvas;
use rt::Object;
use rt::Point;
use rt::Ray;
use rt::Sphere;
use rt::Vector;
use rt::Canvas;

use rt::SubPoint;

pub fn main_loop(canvas: &mut Canvas) { 

    let wall_z = 10.;

    let pixel_size = canvas.wall / canvas.width as f32;
    let half = canvas.wall / 2.;

    let mut obj = Object::new(Box::new(Sphere::new(1.)));
    let scale_v = Vector::new([1., 0.5, 1.]);
    obj.scale(&scale_v);
    let red = Color::new([1., 0., 0.]);

    for y in 0..canvas.width - 1 {
        let world_y = -half + pixel_size * y as f32;
        for x in 0..canvas.height - 1 {
            let world_x = -half + pixel_size * x as f32;
            let position = Point::new([world_x, world_y, wall_z]);
            let r = Ray::new(canvas.camera_origin, (position.sub(canvas.camera_origin)).normalize());
        
            let result = obj.intersect(&r);
            if result.is_some() {
                canvas.write_pixel(x, y, red);
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

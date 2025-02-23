use rt::Color;
// use rt::structure::canvas::Canvas;
use rt::Object;
use rt::Point;
use rt::Ray;
use rt::Sphere;
use rt::Vector;
use rt::Canvas;

use std::time::Duration;
use minifb::Key;

pub fn main_loop(canvas: &mut Canvas) {


}

fn main() {

    let mut _canvas = Canvas::new(300, 200);
    _canvas.canvas_loop(main_loop, 16);
    
    let obj = Object::new(Box::new(Sphere::new(1.)));

    let ray = Ray::new(Point::new([0., 0., 5.]), Vector::new([0., 0., 1.]));

    let result = obj.intersect(&ray);

    match result {
        Some(_) => println!("Intersection"),
        None => println!("No intersection"),
    }
}

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

fn main() {

    let mut _canvas = Canvas::new(300, 200);
    let obj = Object::new(Box::new(Sphere::new(1.)));

    while _canvas.window.is_open() && !_canvas.window.is_key_down(Key::Escape) {

        _canvas.update_window();

        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }


    let ray = Ray::new(Point::new([0., 0., 5.]), Vector::new([0., 0., 1.]));

    let result = obj.intersect(&ray);

    match result {
        Some(_) => println!("Intersection"),
        None => println!("No intersection"),
    }
}

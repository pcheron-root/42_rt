use rt::Object;
use rt::Point;
use rt::Ray;
use rt::Sphere;
use rt::Vector;

fn main() {
    let position = Point::new([0., 0., 0.]);
    let obj = Object::new(Box::new(Sphere::new(1.)), position);

    let ray = Ray::new(Point::new([0., 0., 5.]), Vector::new([0., 0., 1.]));

    let result = obj.intersect(&ray);

    match result {
        Some(_) => println!("Intersection"),
        None => println!("No intersection"),
    }
}

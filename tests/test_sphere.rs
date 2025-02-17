use rt::Object;
use rt::Point;
use rt::Ray;
use rt::Sphere;
use rt::Vector;

#[test]
fn test_sphere_intersection() {
    let position = Point::new([0., 0., 0.]);
    let obj = Object::new(Box::new(Sphere::new(1.)), position);

    let ray = Ray::new(Point::new([0., 0., 5.]), Vector::new([0., 0., -1.]));

    let result = obj.intersect(&ray);

    assert!(result.is_some(), "Expected an intersection, but got none");
}

#[test]
fn test_sphere_no_intersection() {
    let position = Point::new([0., 0., 0.]);
    let obj = Object::new(Box::new(Sphere::new(1.)), position);

    let ray = Ray::new(Point::new([0., 0., 5.]), Vector::new([0., 0., 1.]));

    let result = obj.intersect(&ray);

    assert!(result.is_none(), "Expected no intersection, but got one");
}

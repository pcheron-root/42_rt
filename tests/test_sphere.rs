use rt::Object;
use rt::Point;
use rt::Ray;
use rt::Sphere;
use rt::Vector;

#[test]
fn test_sphere_intersection() {
    let obj = Object::new(Box::new(Sphere::new(1.)));

    let ray = Ray::new(Point::new([0., 0., 5.]), Vector::new([0., 0., -1.]));

    let result = obj.intersect(&ray);

    assert!(result.is_some(), "Expected an intersection, but got none");
}

#[test]
fn test_sphere_no_intersection() {
    let obj = Object::new(Box::new(Sphere::new(1.)));

    let ray = Ray::new(Point::new([0., 0., 5.]), Vector::new([0., 0., 1.]));

    let result = obj.intersect(&ray);

    assert!(result.is_none(), "Expected no intersection, but got one");
}

#[test]
fn test_sphere_intersection_with_translation() {
    let mut obj = Object::new(Box::new(Sphere::new(1.)));
    
    let translation = Vector::new([2., 0., 2.]);
    obj.translate(&translation);

    let ray = Ray::new(Point::new([0., 0., 0.]), Vector::new([1., 0., 1.]).normalize());

    let result = obj.intersect(&ray);

    assert!(result.is_some(), "Expected an intersection, but got none");
}
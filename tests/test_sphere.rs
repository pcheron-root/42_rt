#[cfg(test)]
mod tests {
    use rt::{Object, Point, Ray, Shape, Sphere, Transform, Vector};

    #[test]
    fn test_sphere_intersection() {
        let obj = Object::new(Shape::Sphere(Sphere::new(1.)));

        let ray = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., -1.));

        let result = obj.intersect(ray, 1.);

        assert!(result.is_some(), "Expected an intersection, but got none");
    }

    #[test]
    fn test_sphere_no_intersection() {
        let obj = Object::new(Shape::Sphere(Sphere::new(1.)));

        let ray = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));

        let result = obj.intersect(ray, 1.);

        assert!(result.is_none(), "Expected no intersection, but got one");
    }

    #[test]
    fn test_sphere_intersection_with_translation() {
        let mut obj = Object::new(Shape::Sphere(Sphere::new(1.)));
        obj.translate(Vector::new(2., 0., 2.));

        let ray = Ray::new(Point::new(0., 0., 0.), Vector::new(1., 0., 1.).normalize());

        let result = obj.intersect(ray, 1.);

        assert!(result.is_some(), "Expected an intersection, but got none");
    }

    #[test]
    fn test_sphere_intersection_0() {
        let obj = Object::new(Shape::Sphere(Sphere::new(1.)));

        let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.).normalize());
        let result = obj.intersect(ray, 1.).unwrap();
        assert_eq!(result.t, 4.);
    }

    #[test]
    fn test_sphere_intersection_1() {
        let obj = Object::new(Shape::Sphere(Sphere::new(1.)));

        let ray = Ray::new(Point::new(0., 1., -5.), Vector::new(0., 0., 1.).normalize());
        let result = obj.intersect(ray, 1.).unwrap();
        assert_eq!(result.t, 5.);
    }

    #[test]
    fn test_ray_miss_sphere() {
        let obj = Object::new(Shape::Sphere(Sphere::new(1.)));

        let ray = Ray::new(Point::new(0., 2., -5.), Vector::new(0., 0., 1.).normalize());
        let result = obj.intersect(ray, 1.).is_none();
        assert_eq!(result, true);
    }

    #[test]
    fn test_ray_origin_in_sphere() {
        let obj = Object::new(Shape::Sphere(Sphere::new(1.)));

        let ray = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.).normalize());
        let result = obj.intersect(ray, 1.).unwrap();
        assert_eq!(result.t, 1.);
    }

    #[test]
    fn test_ray_miss_sphere_2() {
        let obj = Object::new(Shape::Sphere(Sphere::new(1.)));

        let ray = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.).normalize());
        let result = obj.intersect(ray, 1.).is_none();
        assert_eq!(result, true);
    }

    #[test]
    fn test_sphere_intersection_impact_point() {
        let obj = Object::new(Shape::Sphere(Sphere::new(1.)));

        let ray = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.).normalize());
        let result = obj.intersect(ray, 1.).is_none();
        assert_eq!(result, true);
    }
}

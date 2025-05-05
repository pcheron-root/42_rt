#[cfg(test)]
mod tests {
    use rt::{Cube, Object, Point, Ray, Transformable, Vector};

    #[test]
    fn test_cube_intersection_0() {
        let obj = Object::new(Box::new(Cube::new(1.0)));
        let ray = Ray::new(Point::new(0.0, 0.0, 10.0), Vector::new(0.0, 0.0, -1.0));

        let intersection = obj.intersect(ray);

        assert!(intersection.is_some(), "Should intersect the cube");
    }

    #[test]
    fn test_cube_intersection_1() {
        let obj = Object::new(Box::new(Cube::new(1.0)));
        let ray = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, -1.0, 0.0));

        let intersection = obj.intersect(ray);

        assert!(intersection.is_some(), "Should intersect the cube");
    }

    #[test]
    fn test_cube_intersection_2() {
        let obj = Object::new(Box::new(Cube::new(1.0)));
        let ray = Ray::new(Point::new(10.0, 0.0, 0.0), Vector::new(-1.0, 0.0, 0.0));

        let intersection = obj.intersect(ray);

        assert!(intersection.is_some(), "Should intersect the cube");
    }

    #[test]
    fn test_cube_intersection_with_translation() {
        let mut obj = Object::new(Box::new(Cube::new(1.0)));
        obj.translate(Vector::new(0.0, 5.0, 0.0));
        let ray = Ray::new(Point::new(10.0, 5.0, 0.0), Vector::new(-1.0, 0.0, 0.0));

        let intersection = obj.intersect(ray);

        assert!(intersection.is_some(), "Should intersect the cube");
    }
}

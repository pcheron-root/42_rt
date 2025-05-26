#[cfg(test)]
mod tests {
    use rt::{Intersect, Object, Plane, Point, Ray, Shape, Vector};

    #[test]
    fn test_plane_intersection_from_above() {
        let plane = Object::new(Shape::Plane(Plane::new()));
        let ray = Ray::new(Point::new(0., 1., 0.), Vector::new(0., -1., 0.));

        let hit = plane.intersect(ray, 1.);

        assert!(hit.is_some(), "Ray must hit plane");

        let hit = hit.unwrap();
        assert_eq!(hit.t, 1.);
    }

    #[test]
    fn test_plane_intersection_from_below() {
        let plane = Object::new(Shape::Plane(Plane::new()));
        let ray = Ray::new(Point::new(0., -1., 0.), Vector::new(0., 1., 0.));

        let hit = plane.intersect(ray, 1.);

        assert!(hit.is_some(), "Ray must hit plane");

        let hit = hit.unwrap();
        assert_eq!(hit.t, 1.);
    }

    #[test]
    fn test_plane_no_intersection() {
        let plane = Object::new(Shape::Plane(Plane::new()));
        let ray = Ray::new(Point::new(0., 10., 0.), Vector::new(0., 0., 1.));

        let hit = plane.intersect(ray, 1.);

        assert!(hit.is_none(), "Ray must not hit plane");
    }

    #[test]
    fn test_plane_normal() {
        let plane = Plane::new();

        let n1 = plane.normal_at(Point::new(0., 0., 0.));
        let n2 = plane.normal_at(Point::new(10., 0., -10.));
        let n3 = plane.normal_at(Point::new(-5., 0., 150.));

        let normal = Vector::new(0., 1., 0.);
        assert_eq!(n1, normal);
        assert_eq!(n2, normal);
        assert_eq!(n3, normal);
    }
}

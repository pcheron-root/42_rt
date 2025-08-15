#[cfg(test)]
mod tests {
    use rt::{Object, Point, Ray, Shape, Torus, Vector};

    #[test]
    fn test_torus_intersection() {
        let obj = Object::new(Shape::Torus(Torus::new(2., 1.)));

        let ray = Ray::new(Point::new(5., 0., 0.), Vector::new(-1., 0., 0.));

        let hit = obj.intersect(ray, 1.);

        assert!(hit.is_some(), "Expected an intersection, but got none");
        let hit = hit.unwrap();
        assert_eq!(hit.t, 2.0);
    }
}

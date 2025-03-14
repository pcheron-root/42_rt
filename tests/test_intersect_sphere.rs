#[cfg(test)]
mod tests {
    use rt::{Intersect, Point, Ray, Sphere, Vector};

    #[test]
    fn test_intersect_sphere_0() {
        let origin = Point::new(1., 0., 0.);
        let dir = Vector::new(1., 0., 0.);

        let ray = Ray::new(origin, dir);

        let sphere = Sphere::new(1.0);

        let impact = sphere.intersect(ray).unwrap();

        assert_eq!(impact.normal.x, 1.);
        assert_eq!(impact.normal.y, 0.);
        assert_eq!(impact.normal.z, 0.);
    }

    // #[test]
    // fn test_intersect_sphere_1() {

    //     let origin = Point::new(0., 1., 0.);
    //     let dir = Vector::new(0., 1., 0.);

    //     let ray = Ray::new(origin, dir);

    //     let sphere = Sphere::new(1.0);

    //     let impact = sphere.intersect(&ray).unwrap();

    //     assert_eq!(impact.normal.x, 1.);
    //     assert_eq!(impact.normal.y, 0.);
    //     assert_eq!(impact.normal.z, 0.);
    // }
}

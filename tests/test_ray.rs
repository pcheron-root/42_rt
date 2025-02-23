
#[cfg(test)]
mod tests {
    use rt::{Point, Ray, Vector};


    #[test]
    fn test_position_ray_0() {
        let point = Point::new([2.0, 3.0, 4.0]);
        let vector = Vector::new([1.0, 0.0, 0.0]);
        let ray = Ray::new(point, vector);
        let result = ray.position(1.0);
        assert_eq!(result.data.x, 3.0);
        assert_eq!(result.data.y, 3.0);
        assert_eq!(result.data.z, 4.0);
    }

    #[test]
    fn test_position_ray_1() {
        let point = Point::new([2.0, 3.0, 4.0]);
        let vector = Vector::new([1.0, 0.0, 0.0]);
        let ray = Ray::new(point, vector);
        let result = ray.position(2.5);
        assert_eq!(result.data.x, 4.5);
        assert_eq!(result.data.y, 3.0);
        assert_eq!(result.data.z, 4.0);
    }

    #[test]
    fn test_position_ray_2() {
        let point = Point::new([2.0, 3.0, 4.0]);
        let vector = Vector::new([1.0, 0.0, 0.0]);
        let ray = Ray::new(point, vector);
        let result = ray.position(-1.0);
        assert_eq!(result.data.x, 1.0);
        assert_eq!(result.data.y, 3.0);
        assert_eq!(result.data.z, 4.0);
    }

}

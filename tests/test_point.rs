#[cfg(test)]
mod tests {
    use rt::{Point, Vector};

    #[test]
    fn test_create_point() {
        let p_a = Point::new(1.0, 2.0, 3.0);

        assert_eq!(p_a.x, 1.0);
        assert_eq!(p_a.y, 2.0);
        assert_eq!(p_a.z, 3.0);
    }

    #[test]
    fn test_add_vector_from_point() {
        let p_a = Point::new(1.0, 2.0, 3.0);
        let v_b = Vector::new(3.0, 4.0, 5.0);

        let p_c = p_a + v_b;
        assert_eq!(p_c.x, 4.0);
        assert_eq!(p_c.y, 6.0);
        assert_eq!(p_c.z, 8.0);
    }

    #[test]
    fn test_sub_vector_from_point() {
        let p_a = Point::new(3.0, 2.0, 1.0);
        let v_b = Vector::new(5.0, 6.0, 7.0);

        let p_c = p_a - v_b;
        assert_eq!(p_c.x, -2.0);
        assert_eq!(p_c.y, -4.0);
        assert_eq!(p_c.z, -6.0);
    }

    #[test]
    fn test_sub_point_to_point() {
        let p_a = Point::new(1.0, 2.0, 3.0);
        let p_b = Point::new(3.0, 4.0, 5.0);

        let p_c = p_a - p_b;
        assert_eq!(p_c.x, -2.0);
        assert_eq!(p_c.y, -2.0);
        assert_eq!(p_c.z, -2.0);

        let p_d = Point::new(3.0, 2.0, 1.0);
        let p_e = Point::new(5.0, 6.0, 7.0);

        let p_f = p_d - p_e;
        assert_eq!(p_f.x, -2.0);
        assert_eq!(p_f.y, -4.0);
        assert_eq!(p_f.z, -6.0);
    }
}

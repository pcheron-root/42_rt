#[cfg(test)]
mod tests {
    use rt::Color;

    #[test]
    fn test_create_color() {
        let c_a = Color::new(0.9, 0.6, 0.75);
        let c_b = Color::new(0.7, 0.1, 0.25);

        let c_c = c_a + c_b;

        let r = Color::new(1.6, 0.7, 1.0);

        assert_eq!(r, c_c);
    }

    #[test]
    fn test_sub_color_from_color() {
        let c_a = Color::new(0.9, 0.6, 0.75);
        let c_b = Color::new(0.7, 0.1, 0.25);

        let c_c = c_a - c_b;

        let r = Color::new(0.2, 0.5, 0.5);

        assert_eq!(r, c_c);
    }

    #[test]
    fn test_sub_color_from_zero_color() {
        let c_zero = Color::new(0.0, 0.0, 0.0);
        let c_a = Color::new(1.0, 1.0, 1.0);

        let c_b = c_zero - c_a;

        let r = Color::new(-1., -1., -1.);

        assert_eq!(r, c_b);
    }

    #[test]
    fn test_mult_color_by_scalar() {
        let c_a = Color::new(0.2, 0.3, 0.4);
        let c_b = c_a * 2.0;

        let r = Color::new(0.4, 0.6, 0.8);

        assert_eq!(r, c_b);
    }

    #[test]
    fn test_mult_colors() {
        let c_a = Color::new(1.0, 0.2, 0.4);
        let c_b = Color::new(0.9, 1.0, 0.1);

        let c_b = c_a * c_b;

        let r = Color::new(0.9, 0.2, 0.04);

        assert_eq!(r, c_b);
    }
}

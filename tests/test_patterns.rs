
#[cfg(test)]
mod tests {

    use rt::{Color, Pattern, Point};


    #[test]
    fn test_creating_stripe_pattern() {
        let white = Color::new(1., 1., 1.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(white, black, true, false, false);

        assert_eq!(white.r, pattern.a.r);
        assert_eq!(white.g, pattern.a.g);
        assert_eq!(white.b, pattern.a.b);

        assert_eq!(black.r, pattern.b.r);
        assert_eq!(black.g, pattern.b.g);
        assert_eq!(black.b, pattern.b.b);
    }

    #[test]
    fn test_stripe_pattern_is_const_x() {
        let white = Color::new(1., 1., 1.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(white, black, true, false, false);

        let point = Point::new(0.0, 0.0, 0.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 1.);
        let point = Point::new(0.0, 1.0, 0.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 1.);
        let point = Point::new(0.0, 2.0, 0.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 1.);

        let point = Point::new(0.0, 0.0, 1.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 1.);
        let point = Point::new(0.0, 0.0, 2.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 1.);

        let point = Point::new(0.9, 0.0, 0.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 1.);
        let point = Point::new(1.0, 0.0, 0.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 0.);
        let point = Point::new(-0.1, 0.0, 0.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 0.);
        let point = Point::new(-1.1, 0.0, 0.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 1.);
    }



}

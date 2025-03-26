
#[cfg(test)]
mod tests {

    use rt::{Canvas, Color, Light, Material, Object, Pattern, Point, Shape, Sphere, Vector, World};


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

    #[test]
    fn test_lighting_with_a_pattern_applied() {

        let white = Color::new(1., 1., 1.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(white, black, true, false, false);

        let mut material =Material::new();
        material.pattern = Some(pattern);
        material.ambient = 1.;
        material.diffuse = 0.;
        material.specular = 0.;

        let eyev = Vector::new(0., 0., -1.);
        let normalv = Vector::new(0., 0., -1.);

        let light = Light {
            position: Point::new(0., 0., -10.),
            intensity: Color::new(1., 1., 1.),
        };
        
        
        let p1 = Point::new(0.9, 0., 0.);
        let c1 = Canvas::lighting_ext(&material.clone(), &light, &p1, &eyev, &normalv, false);
        assert_eq!(c1.red(), 1.);
        assert_eq!(c1.green(), 1.);
        assert_eq!(c1.blue(), 1.);
        
        let p2 = Point::new(1.1, 0., 0.);
        let c2 = Canvas::lighting_ext(&material, &light, &p2, &eyev, &normalv, false);
        assert_eq!(c2.red(), 0.);
        assert_eq!(c2.green(), 0.);
        assert_eq!(c2.blue(), 0.);

        
    }

}

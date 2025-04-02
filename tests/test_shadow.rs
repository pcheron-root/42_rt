#[cfg(test)]
mod tests {
    use rt::constants::EPSILON;
    use rt::{
        light_utils::{is_shadowed, shade_it},
        Color, Light, Object, Point, Ray, Shape, Sphere, Transform, Vector, World,
    };

    #[test]
    fn test_light_surface_in_shadow_0() {
        let mut w = World::new();
        let sphere = Object::new(Shape::Sphere(Sphere::new(1.)));

        w.add_object(sphere);

        let light = Light {
            position: Point::new(0., 0., -10.),
            intensity: Color::new(1., 1., 1.),
        };

        w.add_light(light);
        let p = Point::new(0., 10., 0.);
        assert_eq!(is_shadowed(&w, &p), false);
    }

    #[test]
    fn test_light_surface_in_shadow_1() {
        let mut w = World::new();
        let sphere = Object::new(Shape::Sphere(Sphere::new(1.)));
        w.add_object(sphere);
        let light = Light {
            position: Point::new(-10., 10., -10.),
            intensity: Color::new(1., 1., 1.),
        };
        w.add_light(light);

        let p = Point::new(10., -10., 10.);
        assert_eq!(is_shadowed(&w, &p), true);
    }

    #[test]
    fn test_light_surface_in_shadow_2() {
        let mut w = World::new();
        let sphere = Object::new(Shape::Sphere(Sphere::new(1.)));
        w.add_object(sphere);
        let light = Light {
            position: Point::new(10., -10., 10.),
            intensity: Color::new(1., 1., 1.),
        };
        w.add_light(light);

        let p = Point::new(20., -20., 20.);
        assert_eq!(is_shadowed(&w, &p), false);
    }

    #[test]
    fn test_light_surface_in_shadow_3() {
        let mut w = World::new();
        let sphere = Object::new(Shape::Sphere(Sphere::new(1.)));
        w.add_object(sphere);
        let light = Light {
            position: Point::new(10., -10., 10.),
            intensity: Color::new(1., 1., 1.),
        };
        w.add_light(light);

        let p = Point::new(2., -2., 2.);
        assert_eq!(is_shadowed(&w, &p), false);
    }

    #[test]
    fn test_rendering_shadow() {
        let mut w = World::new();
        let mut sphere1 = Object::new(Shape::Sphere(Sphere::new(1.)));
        let mut sphere2 = Object::new(Shape::Sphere(Sphere::new(1.)));

        sphere2.translate(Vector::new(0., 0., 10.));
        sphere1.material.pattern = None;
        sphere2.material.pattern = None;
        w.add_object(sphere1);
        w.add_object(sphere2);
        let light = Light {
            position: Point::new(0., 0., -10.),
            intensity: Color::new(1., 1., 1.),
        };
        w.add_light(light);

        let r = Ray {
            origin: Point::new(0., 0., 5.),
            direction: Vector::new(0., 0., 1.),
        };
        let i = w.intersect(r);

        if i.is_some() {
            let comps = i.unwrap();
            let c = shade_it(&w, &comps);

            assert_eq!(c.red(), 0.1);
            assert_eq!(c.green(), 0.1);
            assert_eq!(c.blue(), 0.1);
        } else {
            assert!(false, "Should intersect");
        }
    }

    #[test]
    fn test_the_hit_should_offset_the_point() {
        let r = Ray {
            origin: Point::new(0., 0., -5.),
            direction: Vector::new(0., 0., 1.),
        };
        let mut sphere1 = Object::new(Shape::Sphere(Sphere::new(1.)));
        sphere1.translate(Vector::new(0., 0., 1.));

        let mut w = World::new();
        w.add_object(sphere1);
        let i = w.intersect(r);

        if i.is_some() {
            let comps = i.unwrap();

            assert_eq!(comps.over_point.x, 0.);
            assert_eq!(comps.over_point.y, 0.);
            assert_eq!(comps.over_point.z < -EPSILON / 2.0, true);
            assert_eq!(comps.point.z > comps.over_point.z, true);
        } else {
            assert_eq!(true, false);
        }
    }
}

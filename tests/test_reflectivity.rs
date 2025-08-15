#[cfg(test)]
mod tests {
    use rt::{
        light_utils::get_phong_color, Material, Object, Plane, Point, Ray, Shape, Sphere,
        Transform, Vector, World,
    };

    #[test]
    fn test_reflective_material() {
        let mat = Material::new();

        assert_eq!(mat.reflective, 0.0);
    }

    #[test]
    fn test_reflecv_calculation() {
        let mut plane = Object::new(Shape::Plane(Plane::new()));
        plane.material = Material::new();

        let origin = Point::new(0., 1., -1.);
        let dir = Vector::new(0., -(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0);
        let ray = Ray::new(origin, dir);
        let impact = plane.intersect(ray, 1.).unwrap();
        assert_eq!(impact.reflectv.x, 0.);
        assert_eq!(impact.reflectv.y, 2.0_f64.sqrt() / 2.0);
        assert_eq!(impact.reflectv.z, 2.0_f64.sqrt() / 2.0);
    }

    #[test]
    fn test_reflected_color_nonreflective_material() {
        let mut world = World::default();
        let mut plane = Object::new(Shape::Sphere(Sphere::new(1.)));
        plane.material = Material::new();
        plane.material.ambient = 0.8;
        world.add_object(plane);

        let origin = Point::new(0., 0., 0.);
        let dir = Vector::new(0., 0., 1.);
        let ray = Ray::new(origin, dir);

        let impact = world.intersect(ray, 1.).unwrap();
        let final_color = get_phong_color(&world, impact);
        assert_eq!(final_color.r, 0.8);
        assert_eq!(final_color.g, 0.8);
        assert_eq!(final_color.b, 0.8);
    }

    #[test]
    fn test_strike_reflective_surface() {
        let mut world = World::default();
        let mut plane = Object::new(Shape::Plane(Plane::new()));
        plane.material = Material::new();
        plane.material.ambient = 0.8;
        plane.material.reflective = 0.1;
        plane.rotate(90.0f64.to_radians(), 0.0, 0.0);
        let mut plane_2 = Object::new(Shape::Plane(Plane::new()));
        plane_2.material = Material::new();

        // plane_2.rotate(pitch, yaw, roll);

        world.add_object(plane);
        world.add_object(plane_2);

        let origin = Point::new(0., 2., -2.);
        let dir = Vector::new(0., -(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0);
        let ray = Ray::new(origin, dir);

        let impact = world.intersect(ray, 1.).unwrap();
        let final_color = get_phong_color(&world, impact);
        assert!(
            final_color.r >= 0.8,
            "Should be little higher than 0.8 and higher"
        );
        assert!(
            final_color.g >= 0.8,
            "Should be little higher than 0.8 and higher"
        );
        assert!(
            final_color.b >= 0.8,
            "Should be little higher than 0.8 and higher"
        );
    }

    #[test]
    fn test_strike_reflective_surface_infinit() {
        let mut world = World::default();
        let mut plane = Object::new(Shape::Plane(Plane::new()));
        plane.material = Material::new();
        plane.material.ambient = 0.8;
        plane.material.reflective = 0.1;
        let vect = Vector::new(0.0, 3.0, 0.0);
        plane.translate(vect);
        let mut plane_2 = Object::new(Shape::Plane(Plane::new()));
        plane_2.material = Material::new();
        plane_2.material.ambient = 0.8;
        plane_2.material.reflective = 0.1;

        // plane_2.rotate(pitch, yaw, roll);

        world.add_object(plane);
        world.add_object(plane_2);

        let origin = Point::new(0., 2., -2.);
        let dir = Vector::new(0., -(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0);
        let ray = Ray::new(origin, dir);

        let impact = world.intersect(ray, 1.).unwrap();
        let final_color = get_phong_color(&world, impact);
        assert!(
            final_color.r > 0.8,
            "Should be little higher than 0.8 and higher than previous test"
        );
        assert!(
            final_color.g > 0.8,
            "Should be little higher than 0.8 and higher than previous test"
        );
        assert!(
            final_color.b > 0.8,
            "Should be little higher than 0.8 and higher than previous test"
        );
    }

    // ajouter la reflected color
    // on fait partir un rayon du point d'impact et qui va en direction du nouveau rayon
    // on multiplie la lumiere qu'on trouve par le coef de reflet
    // #[test]
    // fn test_hit_non_reflective_surface() {
    //     let mut world = World::new();

    //     let sphere1 = Object::new(Shape::Sphere(Sphere::new(3.)));
    //     world.add_object(sphere1);
    // }
    // Given w ← default_world()
    // And r ← ray(point(0, 0, 0), vector(0, 0, 1))
    // And shape ← the second object in w
    // And shape.material.ambient ← 1
    // And i ← intersection(1, shape)
    // When comps ← prepare_computations(i, r)
    // And color ← reflected_color(w, comps)
    // Then color = color(0, 0, 0)
}

// Vacuum: 1
// • Air: 1.00029
// • Water: 1.333
// • Glass: 1.52
// • Diamond: 2.417

// albedo

#[cfg(test)]
mod tests {
    use rt::{
        light_utils::{get_over_point, get_refracted_color},
        Material, Object, Point, Ray, Shape, Sphere, Vector, World,
    };

    #[test]
    fn test_default_material() {
        let mat = Material::new();
        assert_eq!(mat.transparency, 0.);
        assert_eq!(mat.refractive_index, 1.);
    }

    // ce serait pas mal de faire repartir les rayon avec leur indice de refraction
    // on rentre dans une sphere, alors on adopte l'indice de refraction de la sphere, jusqu'a ce qu'on en sorte
    // valable que en cas de transparence, car on traverse l'objet
    #[test]
    fn test_determining_n() {
        let origin = Point::new(1., 0., 0.);
        let dir = Vector::new(1., 0., 0.);

        let ray = Ray::new(origin, dir);

        let sphere = Object::new(Shape::Sphere(Sphere::new(1.0)));
        let mut world = World::new();
        world.add_object(sphere);

        let impact = world.intersect(ray, 1.).unwrap();

        assert_eq!(impact.object.material.refractive_index, 1.);
    }

    #[test]
    fn test_get_over_point() {
        let origin = Point::new(1., 0., 0.);
        let dir = Vector::new(1., 0., 0.);

        let ray = Ray::new(origin, dir);

        let sphere = Object::new(Shape::Sphere(Sphere::new(1.0)));
        let mut world = World::new();
        world.add_object(sphere);

        let impact = world.intersect(ray, 1.).unwrap();
        let overpoint = get_over_point(&impact);
        assert_eq!(overpoint.x, 0.99999976);
        assert_eq!(overpoint.y, 0.);
        assert_eq!(overpoint.z, 0.);
    }

    #[test]
    fn refracted_color_with_no_transparent_material() {
        let origin = Point::new(0., 0., -5.);
        let dir = Vector::new(0., 0., 1.);
        let ray = Ray::new(origin, dir);

        let sphere = Object::new(Shape::Sphere(Sphere::new(1.0)));
        let mut world = World::new();
        world.add_object(sphere);

        let impact = world.intersect(ray, 1.).unwrap();
        assert_eq!(get_refracted_color(&impact, 2).r, 0.);
        assert_eq!(get_refracted_color(&impact, 2).g, 0.);
        assert_eq!(get_refracted_color(&impact, 2).b, 0.);
    }

    #[test]
    fn refracted_color_with_recursive_limit() {
        let origin = Point::new(0., 0., -5.);
        let dir = Vector::new(0., 0., 1.);
        let ray = Ray::new(origin, dir);

        let mut sphere = Object::new(Shape::Sphere(Sphere::new(1.0)));
        sphere.material.transparency = 1.;
        let mut world = World::new();
        world.add_object(sphere);

        let impact = world.intersect(ray, 1.).unwrap();
        assert_eq!(get_refracted_color(&impact, 0).r, 0.);
        assert_eq!(get_refracted_color(&impact, 0).g, 0.);
        assert_eq!(get_refracted_color(&impact, 0).b, 0.);
    }

    // #[test]
    // fn finding_the_reflected_color_under_total_internal_reflection() {
    // let origin = Point::new(0., 0., -5.);
    // let dir = Vector::new(0., 0., 1.);
    // let ray = Ray::new(origin, dir);
    // let mut plane = Object::new(Shape::Plane(Plane::new()));
    // let impact = plane.intersect(ray, 1.).unwrap();

    // let impact = world.intersect(ray, 1.).unwrap();

    // }
}

// 3. Add a new attribute in prepare_computations(), called under_point, which determines where the refracted ray will originate.

// 4. Handle refraction when the surface is opaque.
// 5. Handle refraction when the maximum recursive depth is reached.
// 6. Handle refraction under total internal reflection. (More on that in a bit!)
// 7. Handle refraction in the general case, when the surface is transparent.
// 8. Combine the reflected and refracted colors with the material color to find
// the final surface color.

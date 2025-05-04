
// Vacuum: 1
// • Air: 1.00029
// • Water: 1.333
// • Glass: 1.52
// • Diamond: 2.417

// albedo 


#[cfg(test)]
mod tests {
    use rt::{light_utils::{get_over_point, get_phong_color}, Intersect, Material, Object, Plane, Point, Ray, Shape, Sphere, Transform, Vector, World};

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

        let impact = world.intersect(ray).unwrap();

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

        let impact = world.intersect(ray).unwrap();
        let overpoint = get_over_point(&impact);
        assert_eq!(overpoint.x, 0.99999976);
        assert_eq!(overpoint.y, 0.);
        assert_eq!(overpoint.z, 0.);
    }

    // #[test]
    // fn refracted_color_with_no_transparent_material() {
    //     let origin = Point::new(1., 0., 0.);
    //     let dir = Vector::new(1., 0., 0.);

    //     let ray = Ray::new(origin, dir);

    //     let sphere = Object::new(Shape::Sphere(Sphere::new(1.0)));
    //     let mut world = World::new();
    //     world.add_object(sphere);

    //     let impact = world.intersect(ray).unwrap();

    // }
}




// 3. Add a new attribute in prepare_computations(), called under_point, which determines where the refracted ray will originate.

// 4. Handle refraction when the surface is opaque.
// 5. Handle refraction when the maximum recursive depth is reached.
// 6. Handle refraction under total internal reflection. (More on that in a bit!)
// 7. Handle refraction in the general case, when the surface is transparent.
// 8. Combine the reflected and refracted colors with the material color to find
// the final surface color.


#[cfg(test)]
mod tests {
    use rt::{Material, Object, Plane, Point, Shape, Vector, Ray};

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
        let dir = Vector::new(0., -(2.0_f32.sqrt() / 2.0), 2.0_f32.sqrt() / 2.0);
        let ray = Ray::new(origin, dir);
        let impact = plane.intersect(ray).unwrap();
        assert_eq!(impact.reflectv.x, 0.);
        assert_eq!(impact.reflectv.y, 2.0_f32.sqrt() / 2.0);
        assert_eq!(impact.reflectv.z, 2.0_f32.sqrt() / 2.0);
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

#[cfg(test)]
mod tests {
    use rt::{Color, Material, Object, Point, Shape, Solid, Sphere};

    #[test]
    fn test_setup_material() {
        let material = Material {
            shininess: 1.,
            specular: 1.,
            ambient: 1.,
            diffuse: 1.,
            texture: Box::new(Solid::new(Color::new(1.0, 1.0, 1.0))),
            reflective: 0.,
        };

        let color = material.color_at(&Point::new(0.0, 0.0, 0.0));

        assert_eq!(color.r, 1.);
        assert_eq!(color.g, 1.);
        assert_eq!(color.b, 1.);

        assert_eq!(material.ambient, 1.);
    }

    #[test]
    fn test_default_material() {
        let material = Material::new();

        let color = material.color_at(&Point::new(0.0, 0.0, 0.0));

        assert_eq!(material.ambient, 0.1);
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 1.0);
        assert_eq!(color.b, 1.0);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.);
    }

    #[test]
    fn test_default_material_of_sphere() {
        let sphere = Object::new(Shape::Sphere(Sphere::new(1.)));

        let color = sphere.material.color_at(&Point::new(0.0, 0.0, 0.0));

        assert_eq!(sphere.material.ambient, 0.1);
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 1.0);
        assert_eq!(color.b, 1.0);
        assert_eq!(sphere.material.diffuse, 0.9);
        assert_eq!(sphere.material.specular, 0.9);
        assert_eq!(sphere.material.shininess, 200.);
    }
}

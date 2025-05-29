#[cfg(test)]
mod tests {
    use rt::{Color, Material, Object, Shape, Sphere};

    #[test]
    fn test_setup_material() {
        let color = Color::new(1., 1., 1.);

        let material = Material {
            shininess: 1.,
            specular: 1.,
            color: color,
            ambient: 1.,
            diffuse: 1.,
            pattern: None,
            reflective: 0.,
            refractive_index: 1.,
            transparency: 0.
        };

        assert_eq!(material.color.red(), 1.);
        assert_eq!(material.color.green(), 1.);
        assert_eq!(material.color.blue(), 1.);

        assert_eq!(material.ambient, 1.);
    }

    // 200 -> 10
    #[test]
    fn test_default_material() {
        let material = Material::new();

        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.color.red(), 1.0);
        assert_eq!(material.color.green(), 1.0);
        assert_eq!(material.color.blue(), 1.0);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 10.); // here
    }

    // 200 -> 10
    #[test]
    fn test_default_material_of_sphere() {
        let sphere = Object::new(Shape::Sphere(Sphere::new(1.)));

        assert_eq!(sphere.material.ambient, 0.1);
        assert_eq!(sphere.material.color.red(), 1.0);
        assert_eq!(sphere.material.color.green(), 1.0);
        assert_eq!(sphere.material.color.blue(), 1.0);
        assert_eq!(sphere.material.diffuse, 0.9);
        assert_eq!(sphere.material.specular, 0.9);
        assert_eq!(sphere.material.shininess, 10.); // here
    }
}

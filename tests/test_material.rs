#[cfg(test)]
mod tests {
    use rt::{Color, Material, Sphere, Shape, Object};

    #[test]
    fn test_setup_material() {
        let color = Color::new([1., 1., 1.]);


        let material = Material{
            shininess: 1.,
            specular: 1.,
            color: color,
            ambient: 1.,
            diffuse: 1.,
        };

        assert_eq!(material.color.red(), 1.);
        assert_eq!(material.color.green(), 1.);
        assert_eq!(material.color.blue(), 1.);

        assert_eq!(material.ambient, 1.);
    }

    #[test]
    fn test_default_material() {

        let material = Material::new();

        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.color.red(), 1.0);
        assert_eq!(material.color.green(), 1.0);
        assert_eq!(material.color.blue(), 1.0);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.);
    }

    #[test]
    fn test_default_material_of_sphere() {
        let sphere = Object::new(Shape::Sphere(Sphere::new(1.)));

        assert_eq!(sphere.material.ambient, 0.1);
        assert_eq!(sphere.material.color.red(), 1.0);
        assert_eq!(sphere.material.color.green(), 1.0);
        assert_eq!(sphere.material.color.blue(), 1.0);
        assert_eq!(sphere.material.diffuse, 0.9);
        assert_eq!(sphere.material.specular, 0.9);
        assert_eq!(sphere.material.shininess, 200.);
    }

    
}

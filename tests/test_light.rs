#[cfg(test)]
mod tests {
    use rt::{Color, Light, Material, Object, Point, Shape, Sphere, Vector, World};

    #[test]
    fn test_light_pos_and_intensity() {
        let intensity = Color::new(1., 1., 1.);
        let position = Point::new(0., 0., 0.);

        let light = Light {
            position,
            intensity,
        };

        assert_eq!(light.position.x, 0.);
        assert_eq!(light.position.y, 0.);
        assert_eq!(light.position.z, 0.);
        assert_eq!(light.intensity.r, 1.);
        assert_eq!(light.intensity.g, 1.);
        assert_eq!(light.intensity.b, 1.);
    }

    #[test]
    fn test_light_0() {
        let obj = Object::new(Shape::Sphere(Sphere::new(1.))).material(Material::new());
        let position = Point::new(0., 0., 0.);

        let eyev = Vector::new(0., 0., -1.);
        let normalv = Vector::new(0., 0., -1.);

        let light = Light {
            position: Point::new(0., 0., -10.),
            intensity: Color::new(1., 1., 1.),
        };

        let result = World::lighting(&obj, &light, &position, &eyev, &normalv, false);

        assert_eq!(result.red(), 1.9);
        assert_eq!(result.green(), 1.9);
        assert_eq!(result.blue(), 1.9);
    }

    #[test]
    fn test_light_1() {
        let obj = Object::new(Shape::Sphere(Sphere::new(1.))).material(Material::new());
        let position = Point::new(0., 0., 0.);

        let eyev = Vector::new(0., (2.0_f32).sqrt() / 2.0, (2.0_f32).sqrt() / 2.0);
        let normalv = Vector::new(0., 0., -1.);

        let light = Light {
            position: Point::new(0., 0., -10.),
            intensity: Color::new(1., 1., 1.),
        };

        let result = World::lighting(&obj, &light, &position, &eyev, &normalv, false);

        assert_eq!(result.red(), 1.);
        assert_eq!(result.green(), 1.);
        assert_eq!(result.blue(), 1.);
    }

    #[test]
    fn test_light_2() {
        let obj = Object::new(Shape::Sphere(Sphere::new(1.))).material(Material::new());
        let position = Point::new(0., 0., 0.);

        let eyev = Vector::new(0., 0., -1.);
        let normalv = Vector::new(0., 0., -1.);

        let light = Light {
            position: Point::new(0., 10., -10.),
            intensity: Color::new(1., 1., 1.),
        };

        let result = World::lighting(&obj, &light, &position, &eyev, &normalv, false);

        assert_eq!(result.red(), 0.7363961);
        assert_eq!(result.green(), 0.7363961);
        assert_eq!(result.blue(), 0.7363961);
    }

    #[test]
    fn test_light_3() {
        let obj = Object::new(Shape::Sphere(Sphere::new(1.))).material(Material::new());
        let position = Point::new(0., 0., 0.);

        let eyev = Vector::new(0., -(2.0_f32.sqrt() / 2.0), -(2.0_f32.sqrt() / 2.0));
        let normalv = Vector::new(0., 0., -1.);

        let light = Light {
            position: Point::new(0., 10., -10.),
            intensity: Color::new(1., 1., 1.),
        };

        let result = World::lighting(&obj, &light, &position, &eyev, &normalv, false);

        assert_eq!(result.red(), 1.6363853);
        assert_eq!(result.green(), 1.6363853);
        assert_eq!(result.blue(), 1.6363853);
    }

    #[test]
    fn test_light_4() {
        let obj = Object::new(Shape::Sphere(Sphere::new(1.))).material(Material::new());
        let position = Point::new(0., 0., 0.);

        let eyev = Vector::new(0., 0., -1.);
        let normalv = Vector::new(0., 0., -1.);

        let light = Light {
            position: Point::new(0., 0., 10.),
            intensity: Color::new(1., 1., 1.),
        };

        let result = World::lighting(&obj, &light, &position, &eyev, &normalv, false);

        assert_eq!(result.red(), 0.1);
        assert_eq!(result.green(), 0.1);
        assert_eq!(result.blue(), 0.1);
    }
}

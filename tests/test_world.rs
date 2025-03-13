#[cfg(test)]
mod tests {
    use rt::Object;
    use rt::Shape;
    use rt::Sphere;
    use rt::World;

    #[test]
    fn test_world_create() {
        let mut world = World::new();
        let sphere = Object::new(Shape::Sphere(Sphere::new(1.)));

        world.add_object(sphere);

        assert_eq!(world.objects.len(), 1);
    }
}

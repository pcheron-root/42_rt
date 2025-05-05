#[cfg(test)]
mod tests {
    use rt::{Object, Sphere, World};

    #[test]
    fn test_world_create() {
        let mut world = World::new();
        let sphere = Object::new(Box::new(Sphere::new(1.)));

        world.add_object(sphere);

        assert_eq!(world.objects.len(), 1);
    }
}

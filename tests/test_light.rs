

#[cfg(test)]
mod tests {
    use rt::{Light, Color, Point};


    #[test]
    fn test_light_pos_and_intensity() {
        let intensity = Color::new([1., 1., 1.]);
        let color = Color::new([1., 1., 1.]);
        let position = Point::new([0., 0., 0.]);

        let light = Light{
            position: position,
            color: color,
            intensity: intensity,
        };
        assert_eq!(light.position.data.x, 0.);
        assert_eq!(light.position.data.y, 0.);
        assert_eq!(light.position.data.z, 0.);
        assert_eq!(light.intensity.data.x, 1.);
        assert_eq!(light.intensity.data.y, 1.);
        assert_eq!(light.intensity.data.z, 1.);
        assert_eq!(light.color.data.x, 1.);
        assert_eq!(light.color.data.y, 1.);
        assert_eq!(light.color.data.z, 1.);
    }

    // #[test]
    // fn

}
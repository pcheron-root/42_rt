
#[cfg(test)]
mod tests {
    use rt::{light_utils::is_shadowed, Canvas, Color, Light, Point, Vector};


    // #[test]
    // fn test_light_surface_in_shadow() {

    //     let eyev = Vector::new([0., 0., -1.]);
    //     let normalv = Vector::new([0., 0., -1.]);

    //     let light = Light { position: Point::new([0., 0., -10.]), intensity: Color::new([1., 1., 1.]),};
    
    //     let in_shadow = true;

    //     // let result = Canvas::lighting_ext(material, light, point, eyev, normalv)
    // }

    #[test]
    fn test_light_surface_in_shadow_0() {

        let eyev = Vector::new([0., 0., -1.]);
        let normalv = Vector::new([0., 0., -1.]);

        let light = Light { position: Point::new([0., 0., -10.]), intensity: Color::new([1., 1., 1.]),};
    

        // let w = 

        let p = Point::new([0., 10., 0.]);

        assert_eq!(is_shadowed(&w, &p), false);
    }

    // Scenario: The shadow when an object is between the point and the light
    // Given w ← default_world()
    // And p ← point(10, -10, 10)
    // Then is_shadowed(w, p) is true

    #[test]
    fn test_light_surface_in_shadow() {

        let eyev = Vector::new([0., 0., -1.]);
        let normalv = Vector::new([0., 0., -1.]);

        let light = Light { position: Point::new([0., 0., -10.]), intensity: Color::new([1., 1., 1.]),};
    
        let in_shadow = true;

        // let result = Canvas::lighting_ext(material, light, point, eyev, normalv)
    }

    // Scenario: There is no shadow when an object is behind the light
    // Given w ← default_world()
    // And p ← point(-20, 20, -20)
    // Then is_shadowed(w, p) is false

    #[test]
    fn test_light_surface_in_shadow() {

        let eyev = Vector::new([0., 0., -1.]);
        let normalv = Vector::new([0., 0., -1.]);

        let light = Light { position: Point::new([0., 0., -10.]), intensity: Color::new([1., 1., 1.]),};
    
        let in_shadow = true;

        // let result = Canvas::lighting_ext(material, light, point, eyev, normalv)
    }

    // Scenario: There is no shadow when an object is behind the point
    // Given w ← default_world()
    // And p ← point(-2, 2, -2)
    // Then is_shadowed(w, p) is false

    #[test]
    fn test_light_surface_in_shadow() {

        let eyev = Vector::new([0., 0., -1.]);
        let normalv = Vector::new([0., 0., -1.]);

        let light = Light { position: Point::new([0., 0., -10.]), intensity: Color::new([1., 1., 1.]),};
    
        let in_shadow = true;

        // let result = Canvas::lighting_ext(material, light, point, eyev, normalv)
    }

}








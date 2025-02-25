#[cfg(test)]
mod tests {
    use rt::utils::are_almost_equal;
    use rt::Color;
    
    #[test]
    fn test_create_color() {
        let c_a = Color::new([0.9, 0.6, 0.75]);
        let c_b = Color::new([0.7, 0.1, 0.25]);
        
        let c_c = c_a + c_b;
        
        assert_eq!(are_almost_equal(c_c.data.x, 1.0), true);
        assert_eq!(are_almost_equal(c_c.data.y, 0.7), true);
        assert_eq!(c_c.data.z, 1.0);
    }
    
    #[test]
    fn test_sub_color_from_color() {
        let c_a = Color::new([0.9, 0.6, 0.75]);
        let c_b = Color::new([0.7, 0.1, 0.25]);
        
        let c_c = c_a - c_b;
        
        assert_eq!(are_almost_equal(c_c.data.x, 0.2), true);
        assert_eq!(c_c.data.y, 0.5);
        assert_eq!(c_c.data.z, 0.5);
    }
    
    #[test]
    fn test_sub_color_from_zero_color() {
        let c_zero = Color::new([0.0, 0.0, 0.0]);
        let c_a = Color::new([1.0, 1.0, 1.0]);
        
        let c_b = c_zero - c_a;
        
        assert_eq!(c_b.data.x, 0.0);
        assert_eq!(c_b.data.y, 0.0);
        assert_eq!(c_b.data.z, 0.0);
    }
    
    #[test]
    fn test_mult_color_by_scalar() {
        let c_a = Color::new([0.2, 0.3, 0.4]);
        
        let c_b = c_a * 2.0;
        
        assert_eq!(c_b.data.x, 0.4);
        assert_eq!(c_b.data.y, 0.6);
        assert_eq!(c_b.data.z, 0.8);
    }
    
    #[test]
    fn test_mult_colors() {
        let c_a = Color::new([1.0, 0.2, 0.4]);
        let c_b = Color::new([0.9, 1.0, 0.1]);
        
        let c_b = c_a * c_b;
        
        assert_eq!(c_b.data.x, 0.9);
        assert_eq!(c_b.data.y, 0.2);
        assert_eq!(are_almost_equal(c_b.data.z, 0.04), true);
    }   
}
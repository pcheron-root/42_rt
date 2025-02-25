#[cfg(test)]
mod tests {    
    use rt::{Matrix, Point, Vector};

    #[test]
    fn test_matrix_creation() {
        let m = Matrix::from_col([
            [1., 5.5, 9., 13.5],
            [2., 6.5, 10., 14.5],
            [3., 7.5, 11., 15.5],
            [4., 8.5, 12., 16.5],
        ]);

        assert_eq!(m[0][0], 1.);
        assert_eq!(m[3][0], 4.);
        assert_eq!(m[0][1], 5.5);
        assert_eq!(m[2][1], 7.5);
        assert_eq!(m[2][2], 11.);
    }

    #[test]
    fn test_matrix_equality() {
        let m1 = Matrix::from_col([
            [1., 5., 9., 5.],
            [2., 6., 8., 4.],
            [3., 7., 7., 3.],
            [4., 8., 6., 2.],
        ]);

        let m2 = Matrix::from_col([
            [1., 5., 9., 5.],
            [2., 6., 8., 4.],
            [3., 7., 7., 3.],
            [4., 8., 6., 2.],
        ]);

        assert_eq!(m1, m2);
    }

    #[test]
    fn test_matrix_inequality() {
        let m1 = Matrix::from_col([
            [1., 5., 9., 5.],
            [2., 6., 8., 4.],
            [3., 7., 7., 3.],
            [4., 8., 6., 2.],
        ]);

        let m2 = Matrix::from_col([
            [2., 6., 8., 4.],
            [3., 7., 7., 3.],
            [4., 8., 6., 2.],
            [5., 9., 5., 1.],
        ]);

        assert_ne!(m1, m2);
    }

    #[test]
    fn test_matrix_mul() {
        let m1 = Matrix::from_col([
            [1., 5., 9., 5.],
            [2., 6., 8., 4.],
            [3., 7., 7., 3.],
            [4., 8., 6., 2.],
        ]);

        let m2 = Matrix::from_col([
            [-2., 3., 4., 1.],
            [1., 2., 3., 2.],
            [2., 1., 6., 7.],
            [3., -1., 5., 8.],
        ]);

        let result = Matrix::from_col([
            [20., 44., 40., 16.],
            [22., 54., 58., 26.],
            [50., 114., 110., 46.],
            [48., 108., 102., 42.],
        ]);

        assert_eq!(result, m1 * m2);
    }

    #[test]
    fn test_matrix_point_mul() {
        let m = Matrix::from_col([
            [1., 2., 8., 0.],
            [2., 4., 6., 0.],
            [3., 4., 4., 0.],
            [4., 2., 1., 1.],
        ]);

        let v = Point::new([1., 2., 3.]);

        let result = Point::new([18., 24., 33.]);

        assert_eq!(result, m * v);
    }

    #[test]
    fn test_matrix_vector_mul() {
        let m = Matrix::from_col([
            [1., 2., 8., 0.],
            [2., 4., 6., 0.],
            [3., 4., 4., 0.],
            [4., 2., 1., 1.],
        ]);

        let v = Vector::new([1., 2., 3.]);

        let result = Vector::new([14., 22., 32.]);

        assert_eq!(result, m * v);
    }

    #[test]
    fn test_matrix_mul_by_identity() {
        let m = Matrix::from_col([
            [0., 1., 2., 4.],
            [1., 2., 4., 8.],
            [2., 4., 8., 16.],
            [4., 8., 16., 32.],
        ]);

        let i = Matrix::identity();

        assert_eq!(m, i.clone() * m.clone());
        assert_eq!(m, m.clone() * i.clone());
    }

    #[test]
    fn test_matrix_transpose() {
        let m = Matrix::from_col([
            [0., 9., 1., 0.],
            [9., 8., 8., 0.],
            [3., 0., 5., 5.],
            [0., 8., 3., 8.],
        ]);

        let r = Matrix::from_col([
            [0., 9., 3., 0.],
            [9., 8., 0., 8.],
            [1., 8., 5., 3.],
            [0., 0., 5., 8.],
        ]);

        assert_eq!(r, m.transpose());
    }

    #[test]
    fn test_identity_matrix_transpose() {
        let m = Matrix::identity();

        assert_eq!(m, m.transpose());
    }

    #[test]
    fn test_matrix_inverse_1() {
        let m = Matrix::from_col([
            [8., 7., -6., -3.],
            [-5., 5., 0., 0.],
            [9., 6., 9., -9.],
            [2., 1., 6., -4.],
        ]);

        let i = Matrix::from_col([
            [-0.15385, -0.07692, 0.35897, -0.69231],
            [-0.15385, 0.12308, 0.35897, -0.69231],
            [-0.28205, 0.02564, 0.43590, -0.76923],
            [-0.53846, 0.03077, 0.92308, -1.92308],
        ]);

        assert_eq!(i, m.inverse().unwrap());
    }

    #[test]
    fn test_matrix_inverse_2() {
        let m = Matrix::from_col([
            [9., -5., -4., -7.],
            [3., -2., 9., 6.],
            [0., -6., 6., 6.],
            [9., -3., 4., 2.],
        ]);

        let i = Matrix::from_col([
            [-0.04074, -0.07778, -0.02901, 0.17778],
            [-0.07778, 0.03333, -0.14630, 0.06667],
            [0.14444, 0.36667, -0.10926, -0.26667],
            [-0.22222, -0.33333, 0.12963, 0.33333],
        ]);

        assert_eq!(i, m.inverse().unwrap());
    }

    #[test]
    fn test_point_translation() {
        let vt = Vector::new([5., -3., 2.]);
        let t = Matrix::translation(&vt);

        let p = Point::new([-3., 4., 5.]);

        let r = Point::new([2., 1., 7.]);

        assert_eq!(r, t * p);
    }

    #[test]
    fn test_vector_translation() {
        let vt = Vector::new([5., -3., 2.]);
        let t = Matrix::translation(&vt);

        let v = Vector::new([-3., 4., 5.]);

        assert_eq!(v, t * v.clone());
    }

    #[test]
    fn test_point_scaling() {
        let vs = Vector::new([-1., 1., 1.]);
        let s = Matrix::scaling(&vs);

        let p = Point::new([2., 3., 4.]);

        let r = Point::new([-2., 3., 4.]);

        assert_eq!(r, s * p);
    }

    #[test]
    fn test_half_quarter_rotation_x() {
        let p: Point = Point::new([0., 1., 0.]);
        let a: f32 = 45.;

        let m = Matrix::rotation_x(a.to_radians());

        let r = Point::new([0., 2f32.sqrt() / 2., 2f32.sqrt() / 2.]);

        assert_eq!(r, m * p);
    }

    #[test]
    fn test_full_quarter_rotation_x() {
        let p: Point = Point::new([0., 1., 0.]);
        let a: f32 = 90.;

        let m = Matrix::rotation_x(a.to_radians());

        let r = Point::new([0., 0., 1.]);

        assert_eq!(r, m * p);
    }

    #[test]
    fn test_half_quarter_counter_clockwise_rotation_x() {
        let p: Point = Point::new([0., 1., 0.]);
        let a: f32 = 45.;

        let m = Matrix::rotation_x(a.to_radians()).inverse().unwrap();

        let r = Point::new([0., 2f32.sqrt() / 2., -2f32.sqrt() / 2.]);

        assert_eq!(r, m * p);
    }

    #[test]
    fn test_half_quarter_rotation_y() {
        let p: Point = Point::new([0., 0., 1.]);
        let a: f32 = 45.;

        let m = Matrix::rotation_y(a.to_radians());

        let r = Point::new([2f32.sqrt() / 2., 0., 2f32.sqrt() / 2.]);

        assert_eq!(r, m * p);
    }

    #[test]
    fn test_full_quarter_rotation_y() {
        let p: Point = Point::new([0., 0., 1.]);
        let a: f32 = 90.;

        let m = Matrix::rotation_y(a.to_radians());

        let r = Point::new([1., 0., 0.]);

        assert_eq!(r, m * p);
    }

    #[test]
    fn test_half_quarter_counter_clockwise_rotation_y() {
        let p: Point = Point::new([0., 0., 1.]);
        let a: f32 = 45.;

        let m = Matrix::rotation_y(a.to_radians()).inverse().unwrap();

        let r = Point::new([-2f32.sqrt() / 2., 0., 2f32.sqrt() / 2.]);

        assert_eq!(r, m * p);
    }

    #[test]
    fn test_half_quarter_rotation_z() {
        let p: Point = Point::new([0., 1., 0.]);
        let a: f32 = 45.;

        let m = Matrix::rotation_z(a.to_radians());

        let r = Point::new([-2f32.sqrt() / 2., 2f32.sqrt() / 2., 0.]);

        assert_eq!(r, m * p);
    }

    #[test]
    fn test_full_quarter_rotation_z() {
        let p: Point = Point::new([0., 1., 0.]);
        let a: f32 = 90.;

        let m = Matrix::rotation_z(a.to_radians());

        let r = Point::new([-1., 0., 0.]);

        assert_eq!(r, m * p);
    }

    #[test]
    fn test_half_quarter_counter_clockwise_rotation_z() {
        let p: Point = Point::new([0., 1., 0.]);
        let a: f32 = 45.;

        let m = Matrix::rotation_z(a.to_radians()).inverse().unwrap();

        let r = Point::new([2f32.sqrt() / 2., 2f32.sqrt() / 2., 0.]);

        assert_eq!(r, m * p);
    }

    #[test]
    fn test_shearing_xy_point() {
        let s = Matrix::shearing(1., 0., 0., 0., 0., 0.);
        let p = Point::new([2., 3., 4.]);
        
        let r = Point::new([5., 3., 4.]);

        assert_eq!(r, s * p);
    }

    #[test]
    fn test_shearing_xz_point() {
        let s = Matrix::shearing(0., 1., 0., 0., 0., 0.);
        let p = Point::new([2., 3., 4.]);
        
        let r = Point::new([6., 3., 4.]);

        assert_eq!(r, s * p);
    }

    #[test]
    fn test_shearing_yx_point() {
        let s = Matrix::shearing(0., 0., 1., 0., 0., 0.);
        let p = Point::new([2., 3., 4.]);
        
        let r = Point::new([2., 5., 4.]);

        assert_eq!(r, s * p);
    }

    #[test]
    fn test_shearing_yz_point() {
        let s = Matrix::shearing(0., 0., 0., 1., 0., 0.);
        let p = Point::new([2., 3., 4.]);
        
        let r = Point::new([2., 7., 4.]);

        assert_eq!(r, s * p);
    }

    #[test]
    fn test_shearing_zx_point() {
        let s = Matrix::shearing(0., 0., 0., 0., 1., 0.);
        let p = Point::new([2., 3., 4.]);
        
        let r = Point::new([2., 3., 6.]);

        assert_eq!(r, s * p);
    }

    #[test]
    fn test_shearing_zy_point() {
        let s = Matrix::shearing(0., 0., 0., 0., 0., 1.);
        let p = Point::new([2., 3., 4.]);
        
        let r = Point::new([2., 3., 7.]);

        assert_eq!(r, s * p);
    }

    #[test] 
    fn test_view_matrix() {
        let from = Point::new([1., 3., 2.]);
        let to = Point::new([4., -2., 8.]);
        let up = Vector::new([1., 1., 0.]);

        let t = Matrix::view(from, to, up);

        let r = Matrix::from_col([
            [-0.50709,  0.76772, -0.35857, 0.00000],
            [ 0.50709,  0.60609,  0.59761, 0.00000],
            [ 0.67612,  0.12122, -0.71714, 0.00000],
            [-2.36643, -2.82843,  0.00000, 1.00000],
        ]);

        assert_eq!(r, t);
    }
}
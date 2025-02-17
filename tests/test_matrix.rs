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

    let result = Vector::new([18., 24., 33.]);

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

    assert_eq!(m, i * m);
    assert_eq!(m, m * i);
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
        [ 8., 7., -6., -3.],
        [-5., 5.,  0.,  0.],
        [ 9., 6.,  9., -9.],
        [ 2., 1.,  6., -4.],
    ]);

    let i = Matrix::from_col([
        [-0.15385, -0.07692, 0.35897, -0.69231],
        [-0.15385,  0.12308, 0.35897, -0.69231],
        [-0.28205,  0.02564, 0.43590, -0.76923],
        [-0.53846,  0.03077, 0.92308, -1.92308],
    ]);

    assert_eq!(i, m.inverse().unwrap());
}

#[test]
fn test_matrix_inverse_2() {
    let m = Matrix::from_col([
        [9., -5., -4., -7.],
        [3., -2.,  9.,  6.],
        [0., -6.,  6.,  6.],
        [9., -3.,  4.,  2.],
    ]);

    let i = Matrix::from_col([
        [-0.04074, -0.07778, -0.02901,  0.17778],
        [-0.07778,  0.03333, -0.14630,  0.06667],
        [ 0.14444,  0.36667, -0.10926, -0.26667],
        [-0.22222, -0.33333,  0.12963,  0.33333],
    ]);

    assert_eq!(i, m.inverse().unwrap());
}

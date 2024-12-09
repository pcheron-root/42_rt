use rt::Tuple;

#[test]
fn test_create_tuple() {
    let a = Tuple::new([4.3, -4.2, 3.1, 1.0]);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 1.0);
}

// |a - b| < EPSILON

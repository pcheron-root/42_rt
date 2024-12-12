use rt::Vector;

#[test]
fn test_create_vector() {
    let v_a = Vector::new([1.0, 2.0, 3.0]);
    let v_b = Vector::new([3.0, 4.0, 5.0]);

    let v_c = v_a + v_b;

    assert_eq!(v_c.data.x, 4.0);
    assert_eq!(v_c.data.y, 6.0);
    assert_eq!(v_c.data.z, 8.0);
}

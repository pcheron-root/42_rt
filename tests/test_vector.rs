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

#[test]
fn test_sub_vector_from_vector() {
    let v_a = Vector::new([3.0, 2.0, 1.0]);
    let v_b = Vector::new([5.0, 6.0, 7.0]);

    let v_c = v_a - v_b;

    assert_eq!(v_c.data.x, -2.0);
    assert_eq!(v_c.data.y, -4.0);
    assert_eq!(v_c.data.z, -6.0);
}

#[test]
fn test_sub_vector_from_zero_vector() {
    let v_zero = Vector::new([0.0, 0.0, 0.0]);
    let v_a = Vector::new([1.0, -2.0, 3.0]);

    let v_b = v_zero - v_a;

    assert_eq!(v_b.data.x, -1.0);
    assert_eq!(v_b.data.y, 2.0);
    assert_eq!(v_b.data.z, -3.0);
}

#[test]
fn test_magnitude_of_vector_1() {
    let v_a = Vector::new([1.0, 0.0, 0.0]);

    let magn_va = v_a.magnitude();
    assert_eq!(magn_va, 1.0);
}

#[test]
fn test_magnitude_of_vector_2() {
    let v_a = Vector::new([0.0, 1.0, 0.0]);

    let magn_va = v_a.magnitude();
    assert_eq!(magn_va, 1.0);
}

#[test]
fn test_magnitude_of_vector_3() {
    let v_a = Vector::new([0.0, 0.0, 1.0]);

    let magn_va = v_a.magnitude();
    assert_eq!(magn_va, 1.0);
}

#[test]
fn test_magnitude_of_vector_4() {
    let v_a = Vector::new([1.0, 2.0, 3.0]);

    let magn_va = v_a.magnitude();
    let val = 14.0_f32;
    assert_eq!(magn_va, val.sqrt());
}

#[test]
fn test_magnitude_of_vector_5() {
    let v_a = Vector::new([-1.0, -2.0, -3.0]);

    let magn_va = v_a.magnitude();
    let val = 14.0_f32;
    assert_eq!(magn_va, val.sqrt());
}

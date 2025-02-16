use rt::Tuple;
// use traits::type_trait::TypeTrait;
// use rt::TypeTrait;

#[test]
fn test_create_tuple() {
    let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 1.0);
    assert_eq!(a.is_point(), true);
    assert_eq!(a.is_vector(), false);

    let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
    assert_eq!(a.is_point(), false);
    assert_eq!(a.is_vector(), true);
}

#[test]
fn test_negating_tuple() {
    let v_a = Tuple::new( -2.0, 3.0, -4.0, 1.0);

    let v_b = - v_a;

    assert_eq!(v_b.x, 2.0);
    assert_eq!(v_b.y, -3.0);
    assert_eq!(v_b.z, 4.0);
}

#[test]
fn test_mult_tuple_by_scalar() {
    let v_a = Tuple::new( -2.0, 3.0, -4.0, 1.0);

    let v_b = v_a * 3.5;

    assert_eq!(v_b.x, -7.0);
    assert_eq!(v_b.y, 10.5);
    assert_eq!(v_b.z, -14.0);
}

#[test]
fn test_mult_tuple_by_fraction() {
    let v_a = Tuple::new( -2.0, 3.0, -4.0, 1.0);

    let v_b = v_a * 0.5;

    assert_eq!(v_b.x, -1.0);
    assert_eq!(v_b.y, 1.5);
    assert_eq!(v_b.z, -2.0);
}

#[test]
fn test_div_tuple_by_scalar() {
    let v_a = Tuple::new( -2.0, 3.0, -4.0, 1.0);

    let v_b = v_a / 2.0;

    assert_eq!(v_b.x, -1.0);
    assert_eq!(v_b.y, 1.5);
    assert_eq!(v_b.z, -2.0);
}

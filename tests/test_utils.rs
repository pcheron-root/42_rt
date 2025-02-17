
use rt::utils::are_almost_equal;

#[test]
fn test_almost_equal() {
    let result = are_almost_equal(0.123456, 0.123455);
    assert!(result, "Les nombres devraient être considérés comme égaux !");
}


#[test]
fn test_not_almost_equal() {
    let result = are_almost_equal(0.123456, 0.123500);
    assert!(!result, "Les nombres devraient être considérés comme différents !");
}


#[test]
fn test_exactly_equal() {
    let result = are_almost_equal(0.123456, 0.123456);
    assert!(result, "Les nombres devraient être considérés comme égaux !");
}

#[test]
fn test_small_difference() {
    let result = are_almost_equal(0.123456, 0.123457);
    assert!(result, "Les nombres devraient être considérés comme égaux avec une très petite différence !");
}


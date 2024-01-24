use trust::add;
use trust::sub;
use trust::mul;
use trust::div;

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn test_sub() {
    assert_eq!(sub(2, 1), 1);
}

#[test]
fn test_mul() {
    assert_eq!(mul(2, 3), 6);
}

#[test]
fn test_div() {
    assert_eq!(div(6, 3), 2);
}


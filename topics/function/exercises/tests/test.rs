use function::*;

#[test]
fn test_mul() {
    assert_eq!(mul(1, 0), 0);
    assert_eq!(mul(0, 1), 0);
    assert_eq!(mul(10, 2), 20);
    assert_eq!(mul(13, 7), 91);

    // commutativity
    assert_eq!(mul(7, 13), mul(13, 7));

    // identity and zeros
    assert_eq!(mul(42, 1), 42);
    assert_eq!(mul(0, 12345), 0);

    // boundary: multiplying by 1 preserves value
    assert_eq!(mul(u32::MAX, 1), u32::MAX);
}

#[test]
fn test_div() {
    assert_eq!(div(0, 1), 0);
    assert_eq!(div(10, 2), 5);
    assert_eq!(div(13, 7), 1);
}

// Verify that division by zero panics (documented behavior for integer division)
#[test]
#[should_panic]
fn test_div_by_zero_panics() {
    let _ = div(1, 0);
}

const EPSILON: f64 = 1e-6;
pub fn assert_within_epsilon(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < EPSILON,
        "expected {expected}, got {actual} (eps={EPSILON})"
    );
}

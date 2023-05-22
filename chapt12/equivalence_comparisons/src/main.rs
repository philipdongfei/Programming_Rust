fn main() {
    assert_eq!(6.0 / 2.0 == 3.0, true);

    assert!(f64::is_nan(0.0 / 0.0));
    assert_eq!(0.0 / 0.0 == 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 != 0.0 / 0.0, true);
    assert_eq!(0.0 / 0.0 < 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 > 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 <= 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 >= 0.0 / 0.0, false);
}

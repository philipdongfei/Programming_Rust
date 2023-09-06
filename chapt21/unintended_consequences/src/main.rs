macro_rules! bad_assert_eq {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {  // ($left, $right) -> bad_assert_eq!(s.clone(), "a rose")
            (left_val, right_val) => {
                if !(left_val == right_val) {
                    panic!("assertion failed" /*...*/);
                }
            }
        }
    });
}

fn main() {
    let s = "a rose".to_string();
    bad_assert_eq!(s, "a rose");
    println!("confirmed: {} is a rose", s); // error: use of moved value "s"
}

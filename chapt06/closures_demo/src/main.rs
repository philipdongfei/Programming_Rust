fn main() {
    let is_even = |x| x % 2 == 0;
    // let is_even = |x: u64| -> bool x % 2 == 0; // error
    //let is_even = |x: u64| -> bool {x % 2 == 0 }; // ok
    let is_odd = |x: u64| -> bool { x % 2 == 1 };
    assert_eq!(is_even(14), true);
    assert_eq!(is_odd(14), false);
}

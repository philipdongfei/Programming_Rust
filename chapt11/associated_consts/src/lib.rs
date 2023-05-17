use std::ops::Add;

trait Float {
    const ZERO: Self;
    const ONE: Self;
}

impl Float for f32 {
    const ZERO: f32 = 0.0;
    const ONE: f32 = 1.0;
}

impl Float for f64 {
    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;
}

fn add_one<T: Float + Add<Output=T>>(value: T) -> T {
    value + T::ONE
}

fn fib<T: Float + Add<Output=T>>(n: usize) -> T {
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        n => fib::<T>(n - 1) + fib::<T>(n - 2)
    }
}

#[test]
fn test_func() {
    let a: f32 = 5.0;
    let b: f32 = add_one(a);
    assert_eq!(b, 6.0);

    let c: usize = 3;
    let d: f32 = fib(c);
    assert_eq!(d, 2.0);
}

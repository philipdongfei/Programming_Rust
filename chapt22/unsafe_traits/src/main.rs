// Zeroable was unstable and moved to internal-only use in the num crate in Rust 1.26, but it's a
// good, simple, real-world example.
use core::nonzero::Zeroable;

fn zeroed_vector<T>(len: usize) -> Vec<T>
    where T: Zeroable
{
    let mut vec = Vec::with_capacity(len);
    unsafe {
        std::ptr::write_bytes(vec.as_mut_ptr(), 0, len);
        vec.set_len(len);
    }
    vec
}

#[test]
fn test_zeroed_vec()
{
    let v: Vec<usize> = zeroed_vector(100_000);
    assert!(v.iter().all(|&u| u == 0));
}

fn main() {
    println!("Hello, world!");

    // crashes
    struct HoldsRef<'a>(&'a mut i32);

    unsafe impl<'a> Zeroable for HoldsRef<'a> {}

    let mut v: Vec<HoldsRef> = zeroed_vector(1);
    *v[0].0 = 1; // crashes: dereferences null pointer

}

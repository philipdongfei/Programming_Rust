fn main() {
    println!("test Type Sizes and Alignments!");
}

#[test]
fn test_type_sizes() {
    assert_eq!(std::mem::size_of::<i64>(), 8);
    assert_eq!(std::mem::align_of::<(i32, i32)>(), 4);
}

#[test]
fn test_alignments() {
    // Fat pointers to slices carry their referent's length.
    let slice: &[i32] = &[1, 3, 9, 27, 81];
    assert_eq!(std::mem::size_of_val(slice), 20);

    let text: &str = "alligator";
    assert_eq!(std::mem::size_of_val(text), 9);

    use std::fmt::Display;
    let unremarkable: &dyn Display = &193_u8;
    let remarkable: &dyn Display = &0.0072973525664;

    // These return the size/alignment of the value the
    // trait object points to, not those of the trait object
    // itself. This information comes from the vtable the 
    // trait object refers to.
    assert_eq!(std::mem::size_of_val(unremarkable), 1);
    assert_eq!(std::mem::align_of_val(remarkable), 8);
}

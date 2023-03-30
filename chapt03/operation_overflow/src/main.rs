fn main() {
    /*
    let mut i = 1;
    loop {
        i *= 10; //panic: attempt to multiple with overflow
        // (but only in debug builds!)
        println!("{}\n", i);
    }
    */
    //check operations
    /*
    let mut i: i32 = 1;
    loop {
        // panic: multiplication overflowed (in any build)
        i = i.checked_mul(10).expect("multiplication overflowed");
    }
    */
    // The sum of 10 and 20 can be represented as a u8.
    assert_eq!(10_u8.checked_add(20), Some(30));
    // Unfortunately, the sum of 100 and 200 cannot.
    assert_eq!(100_u8.checked_add(200), None);

    // Do the addition; panic if it overflows.
    let x: u8 = 100;
    let y: u8 = 200;
    // panic because it overflows
    //let sum = x.checked_add(y).unwrap();

    // Oddly, signed division can overflow too, in one particular case.
    // A signed n-bit type can represent -2^(n-1), but not 2^(n-1).
    assert_eq!((-128_i8).checked_div(1), Some(-128_i8));
    assert_eq!((-128_i8).checked_div(-1), None);

    // Wrapping operations
    // The first product can be represented as a u16;
    // the second cannot, so we get 250000 modulo 2^16.
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    // Operations on signed types may wrap to negative values.
    assert_eq!(500_i16.wrapping_mul(500), -12144);

    // In bitwise shift operations, the shift distance
    // is wrapped to fall within the size of the value.
    // So a shift of 17 bits in a 16-bit type is a shift
    // of 1.
    assert_eq!(5_i16.wrapping_shl(17), 10); //0101->1010

    // Saturating operations (no saturating division, remainder, or bitwise shift methods)
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);

    // Overflowing operations return a tuple (result, overflowed), where result is what the
    // wrapping version of the function would return, and overflowed is a bool
    // indicating whether an overflow occurred.
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    // A shift 17 bits is too large for `u16`, and 17 modulo 16 is 1.
    assert_eq!(5_u16.overflowing_shl(17), (10, true));
    assert_eq!(5_u16.overflowing_shl(15), (32768, false));// (0<=N<16):(x, false), (N>=16):(x,true)
    assert_eq!(5_u16.overflowing_shr(17), (2, true));
    assert_eq!(5_u16.overflowing_shr(16), (5, true));
    assert_eq!(5_u16.overflowing_shr(15), (0, false));
    assert_eq!(5_u16.overflowing_shr(14), (0, false));
    assert_eq!(5_u16.overflowing_shr(1), (2, false));

}

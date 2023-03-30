fn main() {
    assert_eq!( 10_i8 as u16, 10_u16 );
    assert_eq!( 2525_u16 as i16, 2525_i16);

    assert_eq!( -1_i16 as i32, -1_i32);
    assert_eq!( 65535_u16 as i32, 65535_i32);

    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);

    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4);

    // error: can't call method `abs`
    //println!("{}", (-4).abs());

    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));

}

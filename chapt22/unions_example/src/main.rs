
union FloatOrInt {
    f: f32,
    i: i32,
}

union SmallOrLarge {
    s: bool,
    l: u64,
}

#[repr(C)]
union SignExtractor {
    value: i64,
    bytes: [u8; 8]
}

fn sign(int: i64) -> bool {
    let se = SignExtractor { value: int };
    println!("{:b} ({:?})", unsafe { se.value }, unsafe { se.bytes });
    unsafe { se.bytes[7] >= 0b10000000 }
}


fn main() {
    println!("test union!");
}

#[test]
fn test_union() {
    // FloatOrInt
    let mut one = FloatOrInt { i: 1 };
    assert_eq!(unsafe { one.i }, 0x00_00_00_01);
    one.f = 1.0;
    assert_eq!(unsafe { one.i }, 0x3F_80_00_00);
    let float = FloatOrInt { f: 31337.0 };
    // prints 1000110111101001101001000000000
    println!("{:b}", unsafe { float.i });
    unsafe {
        match float {
            FloatOrInt { f } => { println!("float {}", f) },
            // warning: unreachable pattern
            FloatOrInt { i } => { println!("int {}", i) }

        }
    }

    // SmallOrLarge
    let u = SmallOrLarge { l: 1337 };
    println!("{}", unsafe {u.l}); // prints 1337
    unsafe {
        match u {
            SmallOrLarge { s: true } => { println!("boolean true"); }
            SmallOrLarge { l: 2 } => { println!("integer 2"); }
            _ => { println!("something else"); }
        }
    }


    // SignExtractor
    assert_eq!(sign(-1), true);
    assert_eq!(sign(1), false);
    assert_eq!(sign(i64::MAX), false);
    assert_eq!(sign(i64::MIN), true);
}

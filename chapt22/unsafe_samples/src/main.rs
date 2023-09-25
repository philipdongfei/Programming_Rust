#[macro_use]
extern crate crashreport;

fn main() {
    // The important bit :)
    crashreport!();

    let mut a: usize = 0;
    let ptr = &mut a as *mut usize;
    unsafe {
        *ptr.offset(5) = 0x7ffff782ff484c;
    }

    // Your panics are now a little fancier!
    panic!("This is a panic!");

}

use lazy_static::lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref HOSTNAME: Mutex<String> = Mutex::new(String::new());
}
/*
static HOSTNAME: Mutex<String> = 
        Mutex::new(String::new()); // error: calls in statics are limited to
*/
                                    // constant functions, tuple structs, and
                                    // tuple variants


struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

const fn mono_to_rgba(level: u8) -> Color {
    Color {
        red: level,
        green: level,
        blue: level,
        alpha: 0xFF
    }
}

const WHITE: Color = mono_to_rgba(255);
const BLACK: Color = mono_to_rgba(000);



fn main() {

    println!("Hello, world!");
}

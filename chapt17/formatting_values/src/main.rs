fn main() {
    println!("{:.3}μs: relocated {} at {:#x} to {:#x}, {} bytes",
        0.84931, "object",
        140737488346304_usize, 6299664_usize, 64);

    println!("number of {}: {}", "elephants", 19);
    println!("from {1} to {0}", "the grave", "the cradle");
    println!("v = {:?}", vec![0,1,2,5,12,29]);
    println!("name = {:?}", "Nemo");
    println!("{:8.2} km/s", 11.186);
    println!("{:20} {:02x} {:02x}", "adc #42", 105, 42);
    println!("{1:02x} {2:02x} {0}", "adc #42", 105, 42);
    println!("{lsb:02x} {msb:02x} {insn}", 
        insn="adc #42", lsb=105, msb=42);
    println!("{msb}", msb=42);
    println!("{:02?}", [110, 11, 9]);
    println!("{:02x?}", [110, 11, 9]);
    assert_eq!(format!("{{a, c}} ⊂ {{a, b, c}}"),
        "{a, c} ⊂ {a, b, c}");
}

use std::rc::Rc;


fn main() {
    // Rust can infer all these types; written out for clarity
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);

    // Rust's memory and thread-safety guarantees depend on ensuring that no value is ever simultaneously shared and mutable.
    //s.push_str(" noodles"); // error: cannot borrow data in an `Rc` as mutable
    
}

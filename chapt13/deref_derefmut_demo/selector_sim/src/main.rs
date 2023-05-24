use selector;
use std::fmt::Display;

fn show_it(thing: &str) { println!("{}", thing); }

fn show_it_generic<T: Display>(thing: T) { println!("{}", thing); }



fn main() {
    let s = selector::Selector { elements: vec!["good", "bad", "ugly"],
            current: 2};

    show_it(&s);

    //show_it_generic(&s); // error: `Selector<&str>` cannot be formatted with the default
    //formatter
    show_it_generic(&s as &str);
    // or 
    show_it_generic(&*s);
}

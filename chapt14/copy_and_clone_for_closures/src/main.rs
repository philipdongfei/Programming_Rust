fn main() {
    let y = 10;
    let add_y = |x| x + y;
    let copy_of_add_y = add_y; // this closure is `Copy`, so ...
    assert_eq!(add_y(copy_of_add_y(22)), 42); // ... we can call both.

    ///////
    /*
    let mut x = 0;
    let mut add_to_x = |n| { x += n; x };

    let copy_of_add_to_x = add_to_x; // this moves, rather than copies
    assert_eq!(add_to_x(copy_of_add_to_x(1)), 2); // error: use of moved value

    */
    /////
    let mut greeting = String::from("Hello, ");
    let greet = move |name| {
        greeting.push_str(name);
        println!("{}", greeting);
    };
    greet.clone()("Alfred");
    greet.clone()("Bruce");


}

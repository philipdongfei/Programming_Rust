use std::fmt::Display;

struct RcBox<T: ?Sized> {
    ref_count: usize,
    value: T,
}

fn display(boxed: &RcBox<dyn Display>) {
    println!("For your enjoyment: {}", &boxed.value);
}

fn main() {
    let boxed_lunch: RcBox<String> = RcBox {
        ref_count: 1,
        value: "lunch".to_string()
    };


    let boxed_displayable: &RcBox<dyn Display> = &boxed_lunch;

    display(&boxed_lunch);

}

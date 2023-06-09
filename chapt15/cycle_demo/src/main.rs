use std::any::type_name;

fn type_of<T>(_: &T) -> &'static str{
    type_name::<T>()
}


fn main() {

    let dirs = ["North", "East", "South", "West"];
    let mut spin = dirs.iter().cycle();
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
    assert_eq!(spin.next(), Some(&"South"));
    assert_eq!(spin.next(), Some(&"West"));
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
    assert_eq!(spin.next(), Some(&"South"));


    use std::iter::{once, repeat};

    // in which the players take turns counting, replacing any number divisible by three with
    // the word fizz(_ _ fizz _ _ fizz ...)
    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    // any number divisible by five with buzz (_ _ _ _ buzz _ _ _ _ buzz ...)
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    // zip fizzes and buzzes
    // (_, _) (_, _) (fizz, _) (_, _) (_, buzz) ...
    let fizzes_buzzes = fizzes.zip(buzzes);

    // replace any number divisible by both 3 and 5 with the word fizzbuzz
    let fizz_buzz = (1..100).zip(fizzes_buzzes)
        .map(|tuple|
            match tuple {
                (i, ("", "")) => i.to_string(),
                (_, (fizz, buzz)) => format!("{}{}", fizz, buzz)
            });
    
    println!("fizz_buzz type: {}", type_of(&fizz_buzz));
    println!("{:?}", fizz_buzz);
    for line in fizz_buzz {
        println!("{}", line);
    }
}

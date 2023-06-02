fn triangle(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
    /*
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
    */
}

fn main() {
    println!("n=10, {}", triangle(10));

    // Iterator.fold examples
    let a = [1, 2, 3];

    // the sum of all of the elements of the array
    let sum = a.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 6);

    // it builds a string, starting with an initial value and continuing with each element from the front until the back:
    let numbers = [1, 2, 3, 4, 5];
    let zero = "0".to_string();

    let result = numbers.iter().fold(zero, |acc, &x| { format!("({acc} + {x})") });
    assert_eq!(result, "(((((0 + 1) + 2) + 3) + 4) + 5)");

    // for -> fold
    let numbers = [1, 2, 3, 4, 5];
    let mut result = 0;

    // for loop:
    for i in &numbers {
        result = result + i;
    }

    // fold:
    let result2 = numbers.iter().fold(0, |acc, &x| acc + x);

    // they're the same
    assert_eq!(result, result2);
}

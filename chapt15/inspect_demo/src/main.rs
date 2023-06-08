fn main() {
    let upper_case: String = "große".chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!(" after:     {:?}", c))
        .collect();
    assert_eq!(upper_case, "GROSSE");

    // Basic usage:
    let a = [1, 4, 2, 3];

    // this iterator sequence is complex.
    let sum = a.iter()
        .cloned()
        .filter(|x| x % 2 == 0)
        .fold(0, |sum, i| sum + i);

    println!("{sum}");

    // let's add some inspect() calls to investigate what's happening
    let sum = a.iter()
        .cloned()
        .inspect(|x| println!("about to filter: {x}"))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("made it through filter: {x}"))
        .fold(0, |sum, i| sum + i);

    println!("{sum}");

    // Logging errors before discarding them:
    let lines = ["1", "2", "a"];

    let sum: i32 = lines
        .iter()
        .map(|line| line.parse::<i32>())
        .inspect(|num| {
            if let Err(ref e) = *num {
                println!("Parsing error: {e}");
            }
        })
        .filter_map(Result::ok)
        .sum();

    println!("Sum: {sum}");

}

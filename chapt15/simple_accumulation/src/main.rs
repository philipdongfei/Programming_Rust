use std::io::prelude::*;


fn main() {

    // count, sum, product
    let stdin = std::io::stdin();
    println!("{}", stdin.lock().lines().count());

    fn triangle(n: u64) -> u64 {
        (1..=n).sum()
    }
    assert_eq!(triangle(20), 210);

    fn factorial(n: u64) -> u64 {
        (1..=n).product()
    }
    println!("n=4: {}", factorial(4));
    assert_eq!(factorial(20), 2432902008176640000);

    // max, min
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().max(), Some(&1));
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().min(), Some(&-5));

    // max_by, min_by
    use std::cmp::Ordering;

    // Compare two f64 values. Panic if given a NaN.
    fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
        lhs.partial_cmp(rhs).unwrap()
    }

    let numbers = [1.0, 4.0, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
    assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));

    let numbers = [1.0, 4.0, std::f64::NAN, 2.0];
    //assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0)); // panics

    // min_by_key, max_by_key
    use std::collections::HashMap;

    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 2);
    populations.insert("Boring", 7_762);
    populations.insert("The Dalles", 15_340);

    assert_eq!(populations.iter().max_by_key(|&(_name, pop)| pop),
        Some((&"Portland", &583_776)));
    assert_eq!(populations.iter().min_by_key(|&(_name, pop)| pop),
        Some((&"Greenhorn", &2)));


    // comparing item sequences
    let packed = "Helen of Troy";
    let spaced = "Helen    of    Troy";
    let obscure = "Helen of Sandusky"; // nice person, just not famous

    assert!(packed != spaced);
    assert!(packed.split_whitespace().eq(spaced.split_whitespace()));

    // This is true because ' ' < 'o'.
    assert!(spaced < obscure);

    // This is true because 'Troy' > 'Sandusky'.
    assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));

    // any and all
    let id = "Iterator";

    assert!(id.chars().any(char::is_uppercase));
    assert!(!id.chars().all(char::is_uppercase));

    // position, rposition 
    let text = "Xerxes";
    assert_eq!(text.chars().position(|c| c == 'e'), Some(1));
    assert_eq!(text.chars().position(|c| c == 'z'), None);
    let bytes = b"Xerxes";
    assert_eq!(bytes.iter().rposition(|&c| c == b'e'), Some(4));
    assert_eq!(bytes.iter().rposition(|&c| c == b'X'), Some(0));

    // fold and rfold
    let a = [5, 6, 7, 8, 9, 10];

    assert_eq!(a.iter().fold(0, |n, _| n+1), 6);   // count
    assert_eq!(a.iter().fold(0, |n, i| n+i), 45);  // sum
    assert_eq!(a.iter().fold(1, |n, i| n*i), 151200); // product

    // max
    assert_eq!(a.iter().cloned().fold(i32::min_value(), std::cmp::max),
        10);

    let a = ["Pack", "my", "box", "with",
            "five", "dozen", "liquor", "jugs"];

    // See also: the `join` method on slices, which won't
    // give you that extra space at the end.
    let pangram = a.iter()
        .fold(String::new(), |s, w| s + w + " ");
    assert_eq!(pangram, "Pack my box with five dozen liquor jugs ");

    let weird_pangram = a.iter()
        .rfold(String::new(), |s, w| s + w + " ");
    assert_eq!(weird_pangram, "jugs liquor dozen five with box my Pack ");


    // nth, nth_back
    let mut squares = (0..10).map(|i| i*i);

    assert_eq!(squares.nth(4), Some(16));
    assert_eq!(squares.nth(0), Some(25));
    assert_eq!(squares.nth(6), None);

    // last
    let squares = (0..10).map(|i| i*i);
    assert_eq!(squares.last(), Some(81));


    // find, rfind, and find_map
    assert_eq!(populations.iter().find(|&(_name, &pop)| pop > 1_000_000), None);
    assert_eq!(populations.iter().find(|&(_name, &pop)| pop > 500_000), Some((&"Portland", &583_776)));
    // basic usage
    let a = [1, 2, 3];

    assert_eq!(a.iter().find(|&&x| x == 2), Some(&2));
    assert_eq!(a.iter().find(|&&x| x == 5), None);

    let a = [1, 2, 3];
    let mut iter = a.iter();
    assert_eq!(iter.find(|&&x| x == 2), Some(&2));

    // we can still use `iter`, as there are more elements.
    assert_eq!(iter.next(), Some(&3));

    //rfind
    // basic usage
    let a = [1, 2, 3];

    assert_eq!(a.iter().rfind(|&&x| x == 2), Some(&2));
    assert_eq!(a.iter().rfind(|&&x| x == 5), None);


    let a = [1, 2, 3];
    let mut iter = a.iter();
    assert_eq!(iter.rfind(|&&x| x == 2), Some(&2));

    // we can still use `iter`, as there are more elements.
    assert_eq!(iter.next_back(), Some(&1));

    // find_map 
    let a = ["lol", "NaN", "2", "5"];
    let first_number = a.iter().find_map(|s| s.parse().ok());

    assert_eq!(first_number, Some(2));

    /* TODO: fix find_volcano_park
    use std::hash::Hash;
    struct Park<'a> {
        name: &'a str,
        state: &'a str,
    }


    let mut parks = HashMap::new();
    parks.insert("Portland", Park {name: "Mt. Tabor Park", 
        state:"Portlan"});
    parks.insert("Washingto", Park {name: "Mount Rainer Nation Park", 
        state: "Washingto"});
    parks.insert("Oregon", Park {name: "Crater Lake National Park", 
        state: "Oregon"});
    fn find_volcano_park<'a, K, V>(city: &K, parks: &'a K) -> Option<&'a V>
        where K: Eq  ,  K: Hash 
    {
        parks.get(city)
    }

    let big_city_with_volcano_park = populations.iter()
        .find_map(|(&city, _)| {
            if let Some(park) = find_volcano_park(city, &parks) {
                // find_map returns this value, so our caller knows
                // *which* park we found.
                return Some((city, park.name));
            }
            // Reject this item, and continue the search.
            None
        });
    assert_eq!(big_city_with_volcano_park, 
        Some(("Portland", "Mt. Tabor Park")));
    */
    

}

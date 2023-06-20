use std::collections::HashSet;

fn main() {
    // Type inference lets us omit an explicit type signature (which
    // would be `HashSet<String>` in this example).
    let mut books = HashSet::new();

    // Add some books.
    books.insert("A Dance with Dragons".to_string());
    books.insert("To Kill a Mockingbird".to_string());
    books.insert("The Odyssey".to_string());
    books.insert("The Great Gatsby".to_string());

    // Check for a specific one.
    if !books.contains("The Winds of Winter") {
        println!("We have {} books, but The Winds of Winter ain't one",
            books.len());
    }
    // Remove a book.
    books.remove("The Odyssey");

    // Iterate over everything.
    for book in &books {
        println!("{book}");
    }
    println!("");

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Viking {
        name: String,
        power: usize,
    }

    let mut vikings = HashSet::new();
    
    vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
    vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
    vikings.insert(Viking { name: "OlaOlaff".to_string(), power: 4 });
    vikings.insert(Viking { name: "Harald".to_string(), power: 8 });

    // Use derived implementation to print the vikings.
    for x in &vikings {
        println!("{x:?}");
    }

    // intersection
    println!("intersection:");
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([4, 2, 3, 4]);

    // Print 2, 3, in arbitrary order.
    for x in a.intersection(&b) {
        println!("{x}");
    }

    // union
    println!("union:");
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([4, 2, 3, 4]);

    // Print 1, 2, 3, 4 in arbitrary order.
    for x in a.union(&b) {
        println!("{x}");
    }

    let union: HashSet<_> = a.union(&b).collect();
    assert_eq!(union, [1, 2, 3, 4].iter().collect());
    
    // difference
    println!("difference:");
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([4, 2, 3, 4]);

    // Can be seen as `a - b`.
    for x in a.difference(&b) {
        println!("{x}");
    }

    let diff: HashSet<_> = a.difference(&b).collect();
    assert_eq!(diff, [1].iter().collect());

    // Note that difference is not symmetric,
    // and `b - a` means something else:
    let diff: HashSet<_> = b.difference(&a).collect();
    assert_eq!(diff, [4].iter().collect());

    // symmetric_difference
    println!("symmetric_difference:");
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([4, 2, 3, 4]);

    // Print 1, 4 in arbitrary order.
    for x in a.symmetric_difference(&b) {
        println!("{x}");
    }

    let diff1: HashSet<_> = a.symmetric_difference(&b).collect();
    let diff2: HashSet<_> = b.symmetric_difference(&a).collect();

    assert_eq!(diff1, diff2);
    assert_eq!(diff1, [1, 4].iter().collect());

    // is_disjoint
    let a = HashSet::from([1, 2, 3]);
    let mut b = HashSet::new();

    assert_eq!(a.is_disjoint(&b), true);
    b.insert(4);
    assert_eq!(a.is_disjoint(&b), true);
    b.insert(1);
    assert_eq!(a.is_disjoint(&b), false);

    // is_subset
    let sup = HashSet::from([1, 2, 3]);
    let mut set = HashSet::new();

    assert_eq!(set.is_subset(&sup), true);
    set.insert(2);
    assert_eq!(set.is_subset(&sup), true);
    set.insert(4);
    assert_eq!(set.is_subset(&sup), false);

    // is_superset
    let sub = HashSet::from([1, 2]);
    let mut set = HashSet::new();

    assert_eq!(set.is_superset(&sub), false);

    set.insert(0);
    set.insert(1);
    assert_eq!(set.is_superset(&sub), false);

    set.insert(2);
    assert_eq!(set.is_superset(&sub), true);

}

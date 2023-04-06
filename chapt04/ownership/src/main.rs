
fn main() {
    print_padovan();

    {
        // Rust's Box type
        let point = Box::new((0.625, 0.5)); // point allocated here
        let label = format!("{:?}", point); // label allocated here
        assert_eq!(label, "(0.625, 0.5)");

    }     // both dropped here

    // struct
    struct Person { name: String, birth: i32 }

    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(),
            birth: 1525 });
    composers.push(Person { name: "Downland".to_string(),
            birth: 1563 });
    composers.push(Person { name: "Lully".to_string(),
            birth: 1632 });
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    
    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    //1. Pop a value off the end of the vector:
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    //2. Move a value out of a given index in the vector,
    // and move the last element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    //3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // Let's see what's left of our vector.
    assert_eq!(v, vec!["101", "104", "substitute"]);

    let v = vec!["liberté".to_string(),
            "égalité".to_string(),
            "fraternité".to_string()];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    {
        struct Person { name: Option<String>, birth: i32 }

        let mut composers = Vec::new();
        composers.push(Person { name: Some("Palestrina".to_string()),
            birth: 1525 });
        // let first_name = std::mem::replace(&mut composers[0].name, None);
        let first_name = composers[0].name.take(); // take has the same effect as the earlier call to replace
        assert_eq!(first_name, Some("Palestrina".to_string()));
        assert_eq!(composers[0].name, None);
    }

}   

/// padovan's pointer, capacity, and length live directly in the stack frame of the print_padovan function; only the vector's buffer is allocated on the heap.
fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("(1..10) = {:?}", padovan);
}               // dropped here

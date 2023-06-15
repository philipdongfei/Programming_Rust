fn main() {
    // Vec<T>
    // Create an empty vector
    let mut numbers: Vec<i32> = vec![];

    // Create a vector with given contents
    let words = vec!["step", "on", "no", "pets"];
    println!("{:?}", words);
    let mut buffer = vec![0u8; 1024]; // 1024 zeroed-out bytes
    println!("{:?}", buffer);

    use std::collections::HashSet;
    let mut books = HashSet::new();
    books.insert("A Dance With Dragons".to_string());
    books.insert("To Kill a Mockingbird".to_string());
    books.insert("The Odyssey".to_string());
    books.insert("The Great Gatsby".to_string());
    // Convert another collection to a vector.
    let my_books = books.into_iter().collect::<Vec<String>>();

    println!("{:?}", my_books);

    // Accessing Elements
    if let Some(item) = words.first() {
        println!("We got first one! {}", item);
    }
    if let Some(item) = words.last() {
        println!("We got lsst one! {}", item);
    }
    let slice = [0, 1, 2, 3];
    assert_eq!(slice.get(2), Some(&2));
    assert_eq!(slice.get(4), None);

    let mut slice = [0, 1, 2, 3];
    {
        let last = slice.last_mut().unwrap(); // type of last: &mut i32
        assert_eq!(*last, 3);
        *last = 100;
    }
    assert_eq!(slice, [0, 1, 2, 100]);
    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(v.to_vec(),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(v[0..6].to_vec(),
        vec![1, 2, 3, 4, 5, 6]);

    // growing and shrinking vectors
    // vec.retain(test)
    let mut vec = vec![1, 2, 3, 4];
    vec.retain(|&x| x % 2 == 0);
    assert_eq!(vec, [2, 4]);

    // because the elements are visited exactly once in the original order,
    // external state may be used to decide which elements to keep.
    let mut vec = vec![1, 2, 3, 4, 5];
    let keep = [false, true, true, false, true];
    let mut iter = keep.iter();
    vec.retain(|_| *iter.next().unwrap());
    assert_eq!(vec, [2, 3, 5]);
    // vec.retain_mut
    let mut vec = vec![1,2,3,4];
    vec.retain_mut(|x| if *x <= 3 {
        *x += 1;
        true
    } else {
        false
    });
    assert_eq!(vec, [2, 3, 4]);

    // vec.dedup
    let mut byte_vec = b"Misssssssissippi".to_vec();
    byte_vec.dedup();
    assert_eq!(&byte_vec, b"Misisipi");

    let mut byte_vec = b"Misssssssissippi".to_vec();

    let mut seen = HashSet::new();
    // This works because .insert() returns false when the set already
    // contains the item we're inserting.
    byte_vec.retain(|r| seen.insert(*r));

    assert_eq!(&byte_vec, b"Misp");

    
    
    // vec.dedup_by
    let mut vec = vec!["foo", "bar", "Bar", "baz", "bar"];
    vec.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    assert_eq!(vec, ["foo", "bar", "baz", "bar"]);

    // vec.dedup_by_key
    let mut vec = vec![10, 20, 21, 30, 20];
    vec.dedup_by_key(|i| *i / 10);
    assert_eq!(vec, [10, 20, 30, 20]);

    // joining
    assert_eq!([[1, 2], [3, 4], [5, 6]].concat(),
        vec![1, 2, 3, 4, 5, 6]);
    assert_eq!([[1, 2], [3, 4], [5, 6]].join(&0),
        vec![1, 2, 0, 3, 4, 0, 5, 6]);

}

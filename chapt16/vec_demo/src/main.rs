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

    // Splitting
    // splitn, rsplitn
    let v = [10, 40, 30, 20, 60, 50];
    println!("splitn:");
    for group in v.splitn(2, |num| *num % 3 == 0){
        println!("{group:?}");
    }
    println!("rsplitn:");
    for rgroup in v.rsplitn(2, |num| *num % 3 == 0){
        println!("{rgroup:?}");
    }
    // chunks
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.chunks(2);
    assert_eq!(iter.next().unwrap(), &['l', 'o']);
    assert_eq!(iter.next().unwrap(), &['r', 'e']);
    assert_eq!(iter.next().unwrap(), &['m']);
    assert!(iter.next().is_none());
    // windows
    let slice = ['r', 'u', 's', 't'];
    let mut iter = slice.windows(3);
    assert_eq!(iter.next().unwrap(), &['r', 'u', 's']);
    assert_eq!(iter.next().unwrap(), &['u', 's', 't']);
    assert!(iter.next().is_none());

    let slice = ['f', 'o', 'o'];
    let mut iter = slice.windows(4);
    assert!(iter.next().is_none());

    // swap
    let mut v = vec!["foo", "bar", "baz", "qux"];

    assert_eq!(v.swap_remove(1), "bar");
    assert_eq!(v, ["foo", "qux", "baz"]);

    assert_eq!(v.swap_remove(0), "foo");
    assert_eq!(v, ["baz", "qux"]);

    // Sorting and Searching
    let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];
    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0]);
    
    let mut v = [5, 4, 1, 3, 2];
    v.sort_by(|a, b| a.cmp(b));
    assert!(v == [1, 2, 3, 4, 5]);

    // reverse sorting
    v.sort_by(|a, b| b.cmp(a));
    assert!(v == [5, 4, 3, 2, 1]);

    let mut v = [-5i32, 4, 1, -3, 2];

    v.sort_by_key(|k| k.abs());
    assert!(v == [1, 2, -3, 4, -5]);

    // binary_search
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

    assert_eq!(s.binary_search(&13), Ok(9));
    assert_eq!(s.binary_search(&4), Err(7));
    assert_eq!(s.binary_search(&100), Err(13));
    let r = s.binary_search(&1);
    assert!(match r { Ok(1..=4) => true, _ => false,  });

    // contains
    let v = [10, 40, 30];
    assert!(v.contains(&30));
    assert!(!v.contains(&50));

    let v = [String::from("hello"), String::from("world")]; // slice of  `String`
    assert!(v.iter().any(|e| e == "hello")); // search with `&str`
    assert!(!v.iter().any(|e| e == "hi"));

    // Comparing Slices
    assert_eq!([1, 2, 3, 4].starts_with(&[1, 2]), true);
    assert_eq!([1, 2, 3, 4].starts_with(&[2, 3]), false);
    assert_eq!([1, 2, 3, 4].ends_with(&[3, 4]), true);

    // Random Elements
    use rand::seq::IteratorRandom;

    let mut rng = rand::thread_rng();

    let faces = "😀😎😐😕😠😢";
    println!("I am {}!", faces.chars().choose(&mut rng).unwrap());

    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut my_vec = ["a", "b", "c", "d", "e"];
    my_vec.shuffle(&mut rng);
    println!("{my_vec:?}");


}

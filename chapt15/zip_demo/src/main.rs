fn main() {
    let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v, vec![(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);

    use std::iter::repeat;

    let endings = vec!["once", "twice", "chicken soup with rice"];
    let rhyme: Vec<_> = repeat("going")
        .zip(endings)
        .collect();
    assert_eq!(rhyme, vec![("going", "once"),
                           ("going", "twice"),
                           ("going", "chicken soup with rice")]);

    // basic usage:
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let mut iter = a1.iter().zip(a2.iter());

    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), Some((&3, &6)));
    assert_eq!(iter.next(), None);
    
    let s1 = &[1, 2, 3];
    let s2 = &[4, 5, 6];

    let mut iter = s1.iter().zip(s2);

    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), Some((&3, &6)));
    assert_eq!(iter.next(), None);

    let enumerate: Vec<_> = "foo".chars().enumerate().collect();

    let zipper: Vec<_> = (0..).zip("foo".chars()).collect();

    assert_eq!((0, 'f'), enumerate[0]);
    assert_eq!((0, 'f'), zipper[0]);

    assert_eq!((1, 'o'), enumerate[1]);
    assert_eq!((1, 'o'), zipper[1]);

    assert_eq!((2, 'o'), enumerate[2]);
    assert_eq!((2, 'o'), zipper[2]);

    // If both iterators have roughly equivalent syntax, it may be more readable to use zip:
    use std::iter::zip;

    let a = [1, 2, 3];
    let b = [2, 3, 4];

    let mut zipped = zip(
        a.into_iter().map(|x| x * 2).skip(1),
        b.into_iter().map(|x| x * 2).skip(1),
    );
    /*// compared to 
    let mut zipped = a
        .into_iter()
        .map(|x| x * 2)
        .skip(1)
        .zip(b.into_iter().map(|x| x * 2).skip(1));
     */
    assert_eq!(zipped.next(), Some((4, 6)));
    assert_eq!(zipped.next(), Some((6, 8)));
    assert_eq!(zipped.next(), None);

}

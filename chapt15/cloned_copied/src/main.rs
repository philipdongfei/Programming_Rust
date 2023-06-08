fn main() {
    let a = ['1', '2', '3', '∞'];

    assert_eq!(a.iter().next(), Some(&'1'));
    assert_eq!(a.iter().cloned().next(), Some('1'));

    // cloned basic usage
    let a = [1, 2, 3];

    let v_cloned: Vec<_> = a.iter().cloned().collect();

    // cloned is the same as .map(|&x| x), for integers
    let v_map: Vec<_> = a.iter().map(|&x| x).collect();

    assert_eq!(v_cloned, vec![1, 2, 3]);
    assert_eq!(v_map, vec![1, 2, 3]);

    let a = [vec![0_u8, 1, 2], vec![3, 4], vec![23]];
    // don't do this:
    let slower: Vec<_> = a.iter().cloned().filter(|s| s.len() == 1).collect();
    assert_eq!(&[vec![23]], &slower[..]);
    // instead call `cloned` late
    let faster: Vec<_> = a.iter().filter(|s| s.len() == 1).cloned().collect();
    assert_eq!(&[vec![23]], &faster[..]);

    // copy basic usage
    let a = [1, 2, 3];

    let v_copied: Vec<_> = a.iter().copied().collect();

    // copied is the same as .map(|&x| x)
    let v_map: Vec<_> = a.iter().map(|&x| x).collect();

    assert_eq!(v_copied, vec![1, 2, 3]);
    assert_eq!(v_map, vec![1, 2, 3]);
}

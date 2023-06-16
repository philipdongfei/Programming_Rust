use std::collections::VecDeque;


fn main() {
    let v = VecDeque::from(vec![1, 2, 3, 4]);
    // make_contiguous
    let mut buf = VecDeque::with_capacity(15);

    buf.push_back(2);
    buf.push_back(1);
    buf.push_front(3);

    // sorting the deque
    buf.make_contiguous().sort();
    assert_eq!(buf.as_slices(), (&[1, 2, 3] as &[_], &[] as &[_]));

    // sorting it in reverse order
    buf.make_contiguous().sort_by(|a, b| b.cmp(a));
    assert_eq!(buf.as_slices(), (&[3, 2, 1] as &[_], &[] as &[_]));

    let mut buf = VecDeque::with_capacity(15);

    buf.push_back(2);
    buf.push_back(1);
    buf.push_front(3);

    buf.make_contiguous();
    if let (slice, &[]) = buf.as_slices() {
        // we can now be sure that `slice` contains all elements of the deque,
        // while still having immutable access to `buf`.
        assert_eq!(buf.len(), slice.len());
        assert_eq!(slice, &[3, 2, 1] as &[_]);
    }

    // as_slices
    let mut deque = VecDeque::new();

    deque.push_back(0);
    deque.push_back(1);
    deque.push_back(2);

    assert_eq!(deque.as_slices(), (&[0, 1, 2][..], &[][..]));

    deque.push_front(10);
    deque.push_front(9);

    assert_eq!(deque.as_slices(), (&[9, 10][..], &[0, 1, 2][..]));

}

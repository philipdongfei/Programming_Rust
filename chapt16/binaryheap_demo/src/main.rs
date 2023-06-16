use std::collections::BinaryHeap;
use std::collections::binary_heap::PeekMut;


fn main() {
    let mut heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4, 12]);
    if let Some(top) = heap.peek_mut() {
        if *top > 10 {
            PeekMut::pop(top);
        }
    }

    assert_eq!(heap.peek(), Some(&9));
    assert_eq!(heap.pop(), Some(9));
    assert_eq!(heap.pop(), Some(8));
    assert_eq!(heap.pop(), Some(6));
    assert_eq!(heap.pop(), Some(5));

    while let Some(task) = heap.pop() {
        println!("{task}");
    }

}

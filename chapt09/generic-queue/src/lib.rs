

/// A first-in, first-out queue of characters.
pub struct Queue<T> {
    older: Vec<T>, // older elements, eldest last.
    younger: Vec<T> // younger elements, youngest last.
}

impl<T>  Queue<T> {
    pub fn new() -> Self {
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    /// Push a character onto the back of a queue.
    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    /// Pop a character off the front of a queue. Return `Some(c)` if there
    /// was a character to pop, or `None` if the queue was empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty(){
                return None;
            }

            // Bring the elements in younger over to older, and put them in 
            // the promised order.
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        // Now older is guaranteed to have something. Vec's pop method
        // already returns an Option, so we're set.
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}




#[test]
fn test() {
    // test push, pop
//    let mut q = Queue { older: Vec::new(), younger: Vec::new() };
    let mut q = Queue::new();

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);

    // test is_empty
    assert!(q.is_empty());
    q.push('☉');
    assert!(!q.is_empty());

    // test split
    let mut q = Queue { older: Vec::new(), younger: Vec::new() };

    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    let (older, younger) = q.split();
    // q is now uninitialized.
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);

    // passing self as a Box, Rc,or Arc
    let mut bq = Box::new(Queue::new());

    // `Queue::push` expects a `&mut Queue`, but `bq` is a `Box<Queue>`.
    // This is fine: Rust borrows a `&mut Queue` from the `Box` for the
    // duration of the call.
    // `fn push(&mut self, c: char)`
    bq.push('■');
}

#[test]
fn test_generic() {
    let mut q = Queue::<char>::new();
    let _ = &mut q;
    drop(q);

    let mut q = Queue::new();
    let mut r = Queue::new();

    q.push("CAD"); // apparently a Queue<&'static str>
    r.push(0.74); // apparently a Queue<f64>

    q.push("BTC"); // Bitcoins per USD, 2019-6
    r.push(13764.0); // Rust fails to detect irrational exuberance
}

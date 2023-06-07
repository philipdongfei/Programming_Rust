use std::iter::Peekable;

fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
    where I: Iterator<Item=char>
{
    let mut n = 0;
    loop {
        match tokens.peek() {
            Some(r) if r.is_digit(10) => {
                n = n * 10 + r.to_digit(10).unwrap();
            }
            _ => return n
        }
        tokens.next();
    }

}
fn main() {
    let mut chars = "226153980,1766319049".chars().peekable();
    assert_eq!(parse_number(&mut chars), 226153980);
    // Look, `parse_number` didn't consume the comma! So we will.
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 1766319049);
    assert_eq!(chars.next(), None);

    // Basic usage
    let xs = [1, 2, 3];

    let mut iter = xs.iter().peekable();

    // peek() lets us see into the future
    assert_eq!(iter.peek(), Some(&&1));
    assert_eq!(iter.next(), Some(&1));

    assert_eq!(iter.next(), Some(&2));

    // we can peek() multiple times, the iterator won't advance
    assert_eq!(iter.peek(), Some(&&3));
    assert_eq!(iter.peek(), Some(&&3));

    assert_eq!(iter.next(), Some(&3));

    // after the iterator is finished, so is peek()
    assert_eq!(iter.peek(), None); 
    assert_eq!(iter.next(), None); 

    // Using peek_mut to mutate the next item without advancing the iterator:
    let xs = [1, 2, 3];

    let mut iter = xs.iter().peekable();

    // `peek_mut()` lets us see into the future
    assert_eq!(iter.peek_mut(), Some(&mut &1));
    assert_eq!(iter.peek_mut(), Some(&mut &1));
    assert_eq!(iter.next(), Some( &1));

    if let Some(mut p) = iter.peek_mut() {
        assert_eq!(*p, &2);
        // put a value into the iterator
        *p = &1000;
    }

    // The value reappears as the iterator continues
    assert_eq!(iter.collect::<Vec<_>>(), vec![&1000, &3]);

}

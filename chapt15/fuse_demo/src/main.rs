fn main() {

    struct Flaky(bool);

    impl Iterator for Flaky {
        type Item = &'static str;
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 {
                self.0 = false;
                Some("totally the last item")
            } else {
                self.0 = true; // D'oh!
                None
            }
        }
    }

    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));

    let mut not_flaky = Flaky(true).fuse();
    assert_eq!(not_flaky.next(), Some("totally the last item"));
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);

    // an iterator which alternates between Some and None
    struct Alternate {
        state: i32,
    }

    impl Iterator for Alternate {
        type Item = i32;

        fn next(&mut self) -> Option<i32> {
            let val = self.state;
            self.state = self.state + 1;

            // if it's even, Some(i32), else None
            if val % 2 == 0 {
                Some(val)
            } else {
                None
            }
        }
    }

    let mut iter = Alternate { state: 0 };

    // we can see our iterator going back and forth
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

    // however, once we fuse it...
    let mut iter = iter.fuse();
    
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), None);

    // it will always return `None` after the first time.
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

}

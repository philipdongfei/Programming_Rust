fn main() {
    let x = 10;
    let r = &x;     // &x is a shared reference to x
    assert!(*r == 10);    // explicitly dereference r

    // &mut
    let mut y = 32;
    let m = &mut y;  // &mut y is a mutable reference to y
    *m += 32;   // explicitly dereference m to set y's value
    assert!(*m == 64);  // and to see y's new value

    // the . operator implicitly dereferences its left operand
    struct Anime { name: &'static str, bechdel_pass: bool }
    let aria = Anime { name: "Aria: The Animation", bechdel_pass: true };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");

    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    // The . operator can also implicitly borrow a reference to its left operand
    let mut v = vec![1973, 1968];
    v.sort();   // implicitly borrows a mutable reference to v
    (&mut v).sort();  // equivalent, but more verbose

    // assigning references
    {
        let x = 10;
        let y = 20;
        let mut r = &x;
        let b = true;

        if b { r = &y; }
        assert!(*r == 10 || *r == 20);
    } 

    // references to references
    {
        struct Point { x: i32, y: i32 }
        let point = Point { x: 1000, y: 729 };
        let r: &Point = &point;
        let rr: &&Point = &r;
        let rrr: &&&Point = &rr;
        assert_eq!(rrr.y, 729);

    }

    // comparing references
    {
        let x = 10;
        let y = 10;

        let rx = &x;
        let ry = &y;

        let rrx = &rx;
        let rry = &ry;

        assert!(rrx <= rry);
        assert!(rrx == rry);

        assert!(rx == ry);  // their referents are equal
        assert!(!std::ptr::eq(rx, ry)); // but occupy different addresses

      //  assert!(rx == rrx);// error: type mismatch: `&i32` vs `&&i32`
        assert!(rx == *rrx); // this is okay

    }

    // Borrowing References to Arbitrary Expressions
    {
        fn factorial(n: usize) -> usize {
            (1..n+1).product()
        }
        let r = &factorial(6);
        // Arithmetic operators can see through one level of references.
        assert_eq!(r + &1009, 1729);
    }

}

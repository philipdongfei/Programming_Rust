
// v should have at least one element.
// fn smallest<'a>(v: &'a [i32]) -> &'a i32 {...}
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r; }
    }
    s
}




fn main() {
    //let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        let s = smallest(&parabola);
        assert_eq!(*s, 0); // fine: parabola still alive
    }
    //assert_eq!(*s, 0); // bad: points to element of dropped array


    {
        // This does not compile.
        struct S<'a> {
            r: &'a i32
        }
        struct D<'a> {
            s: S<'a>
        }

        let s;
        {
            let x = 10;
            s = S { r: &x };
        }
        //assert_eq!(*s.r, 10); // bad: reads from dropped `x`
    }
    // distinct lifetime parameters
    {
        // bad reference
        /*
        struct S<'a> {
            x: &'a i32,
            y: &'a i32
        }
        */
        struct S<'a, 'b> {
            x: &'a i32,
            y: &'b i32
        }

        let x = 10;
        let r;
        {
            let y = 20;
            {
                let s = S { x: &x, y: &y };
                r = s.x;
            }
        }
        println!("{}", r);

        // Your authors tend to try the simplest possible definition first and then loosen restrictions until the code compiles.
        //fn f<'a>(r: &'a i32, s: &'a i32) -> &'a i32 { r  } // perhaps too tight
        fn f<'a, 'b>(r: &'a i32, s: &'b i32) -> &'a i32 { r  } // looser
    }

    // omitting lifetime parameters
    {
        struct S<'a, 'b> {
            x: &'a i32,
            y: &'b i32
        }
         
        // fn sum_r_xy<'a, 'b, 'c>(r: &'a i32, s: S<'b, 'c>) -> i32
        fn sum_r_xy(r: &i32, s: S) -> i32 {
            r + s.x + s.y
        }

        // fn first_third<'a>(point: &'a [i32; 3]) -> (&'a i32, &'a i32)
        fn first_third(point: &[i32; 3]) -> (&i32, &i32) {
            (&point[0], &point[2])
        }

        struct StringTable {
            elements: Vec<String>,
        }

        impl StringTable {
            // fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String>
            fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
                for i in 0 .. self.elements.len() {
                    if self.elements[i].starts_with(prefix) {
                        return Some(&self.elements[i]);
                    }
                }
                None
            }
        }

    }
}

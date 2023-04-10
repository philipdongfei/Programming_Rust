fn main() {
    {
        let r;
        {
            let x = 1;
            r = &x;
        }
        //assert_eq!(*r, 1); // bad: reads memory `x` used to occupy
    }

    {
        let x = 1;
        {
            let r = &x;

            assert_eq!(*r, 1); // The inner lifetime covers the lifetime of r, but is fully enclosed by the lifetime of x.
        }
    }
    let v = vec![1, 2, 3];
    let r = &v[1];
}

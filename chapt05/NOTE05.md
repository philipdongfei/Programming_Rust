# References

## References to Values

## Working with References

## Reference Safety

### Receiving References as Function Arguments

statics' rules:

- Every static must be initialized.
- Mutable statics are inherently not thread-safe (after all, any thread can access a static at any time), and even in single-threaded programs, they can fall prey to other sorts of reentrancy problems.

Conversely, if we do see a function with a signature like *g(p: &i32)* (or with the lifetimes written out, *g<'a>(p: &'a i32)*), we can tell that it **does not** stash its argument p anywhere that will outlive the call.

### Passing References to Functions

### Returning References

    // v should have at least one element.
    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if *r < *s { s = r; }
        }
        s
    }

When a function takes a single reference as an argument and returns a single reference, Rust assumes that the two must have the same lifetime. Writing this out explicitly would give us:

    fn smallest<'a>(v: &'a [i32]) -> &'a i32 { ... }

    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        let s = smallest(&parabola);
        assert_eq!(*s, 0); // fine: parabola still alive
    }


### Structs Containing References

    struct S<'a> { r: &'a i32 }

    struct D<'a> { s: S<'a> }

Now we've shown something similar about types: a type's lifetime parameters always reveal whether it contains references with interesting (that is, non-*'static) lifetimes and what those lifetimes can be.

### Distinct Lifetime Parameters

    fn f<'a>(r: &'a i32, s: &'a i32) -> &'a i32 {r} // perhaps too tight
    fn f<'a, 'b>(r: &'a i32, s: &'b i32) -> &'a i32 {r} // looser

The downside to this is that adding lifetimes can make types and function signatures harder to read. Your authors tend to try the simplest possible definition first and then loosen restrictions until the code compiles.

### Omitting Lifetime Parameters

If you do return references or other types with lifetime parameters, Rust still tries to make the unambiguous cases easy.
If there are multiple lifetimes among your parameters, then there's no natural reason to prefer one over the other for the return value, and Rust makes you spell out what's going on.
If your function is a method on some type and takes its *self* parameter by reference, then that breaks the tie: Rust assumes that *self's* lifetime is the one to give everything in your return value.

    struct StringTable {
        elements: Vec<String>,
    }
    
    impl StringTable {
        fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0 .. self.elements.len() {
        if self.elements[i].starts_with(prefix) {
            return Some(&self.elements[i]);
        }
        }
        None
        }
    }

    fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String>

    
## Sharing Versus Mutation


## Taking Arms Against a Sea of Objects



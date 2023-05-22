# Operator Overloading

## Arithmetic and Bitwise Operators

In practice, however, Rust tends to avoid supporting mixed-type operations.

    use std::ops::Add;

    impl<L, R> Add<Complex<R>> for Complex<L>
        where
            L: Add<R>,
        {
            ...

        }

Since our type parameter L must implement Add<R>, it usually follows that L and R are going to be the same type: there simply aren't that many types available for L that implement anything else.

### Unary Operators

### Binary Operators

However, Rust does not permit the left operand of + to be a &str, to discourage building up long strings by repeatedly concatenating small pieces on the left.(This performs poorly, requiring time quadratic in the findal length of the string.)

### Compound Assignment Operators

The build-in trait for a compound assignment operator is completely independent of the build-in trait for the corresponding binary operator. Implementing *std::ops::Add* does not automatically implement *std::ops::AddAssign*; if you want Rust to permit your type as the lefthand operand of a += operator, you must implement *AddAssign* yourself.


## Equivalence Comparisons

Why is this trait called *PartialEq*? The traditional mathematical definition of an *equivalence relation*, of which equality is one instance, imposes three requirements. For any values x and y:
- If x == y is true, then y == x must be true as well. In other words, swapping the two sides of an equality comparison doesn't affect the result.
- If x == y and y == z, then it must be the case that x == z. Given any chain of values, each equal to the next, each value in the chain is directly equal to every other. Equality is contagious.
- It must always be true that x == x. (PartialEq(f32, f64): NaN values don't equal themselves)


That last requirement might seem to obvious to be worth stating, but this is exactly where things go awry. Rust's f32 and f64 are IEEE standard floating-point values. According to that standard, expressions like 0.0/0.0 and others with no appropriate value must produce special *not-a-number* values, usually referred to as NaN values. The standard further requireds that a NaN value be treated as unequal to every other value-including itself.

    assert!(f64::is_nan(0.0 / 0.0));
    assert_eq!(0.0 / 0.0 == 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 != 0.0 / 0.0, true);
    assert_eq!(0.0 / 0.0 < 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 > 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 <= 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 >= 0.0 / 0.0, false);


So while Rust's == operator meets the first two requirements for equivalence relations, it clearly doesn't meet the third when used on IEEE floating-point values. This is called a *partial equivalence relation*, so Rust uses the name *PartialEq* for the == operator's built-in trait. 


## Ordered Comparisons

Note that *PartialOrd<Rhs>* extends *PartialEq<Rhs>*: you can do ordered comparisons only on types that you can compare for equality.

But if *partial_cmp* returns *None*, that means *self* and *other* are unordered with respect to each other: neither is greater than the other, nor are they equal. Among all of Rust's primitive types, only comparisons between floating-point values ever return *None*: specifically, comparing a NaN (not-a-number) value with anything else returns *None*. 

If you know that values of two types are always ordered with respect to each other, then you can implement the stricter *std::cmp::Ord* trait:

    trait Ord: Eq + PartialOrd<Self> {
        fn cmp(&self, other: &Self) -> Ordering;
    }

The *cmp* method here simply returns an *Ordering*, instead of an *Option<Ordering>* like *partial_cmp: cmp* always declares its arguments equal or indicates their relative order. Almost all types that implement *PartialOrd* should also implement *Ord*. In the standard library, f32 and f64 are the only exceptions to this rule.

    #[derive(Debug, PartialEq)]
    struct Interval<T> {
        lower: T, // inclusive
        upper: T, // exclusive
    }

You'd like to make values of this type partially ordered: one interval is less than another if it falls entirely before the other, with no overlap. If two unequal intervals overlap, they're unordered: some element of each side is less than some element of the other. And two equal intervals are simply equal.

    use std::cmp::{Ordering, PartialOrd};

    impl<T: PartialOrd> PartialOrd<interval<T>> for Interval<T> {
        fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
            if self == other {
                Some(Ordering::Equal)
            } else if self.lower >= other.upper {
                Some(Ordering::Greater)
            } else if self.upper <= other.lower {
                Some(Ordering::Less)
            } else {
                None
            }
        }
    }
    


## Index and IndexMut

Arrays support the [] operator directly, but on any other type, the expression a[i] is normally shorthand for *a.index(i), where index is a method of the std::ops::Index trait. However, if the expression is being assigned to or borrowed mutably, it's instead shorthand for */*.a.index_mut(i)*, a call to the method of the *std::ops::IndexMut* trait.

Note that when we write *image[row][column]*, if *row* is out of bounds, our *.index()* method will try to index *self.pixels* out of range, triggering a panic. This is how *Index* and *IndexMut* implementations are supposed to behave: out-of-bounds access is detected and causes a panic, the same as when you index an array, slice, or vector out of bounds.


## Other Operators



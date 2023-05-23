# Utility Traits

*Language extension traits*
    Just as the operator overloading traits we covered in the previous chapter make it possible for you to use Rust's expression operators on your own types, there are several other standard library traits that serve as Rust extension points, allowing you to integrate your own types more closely with the language.
(**Drop, Deref, DerefMut**, and the conversion traits **From** and **Into**)

*Marker traits*
    These are traits mostly used to bound generic type variables to express constraints you can't capture otherwise.(**Sized, Copy**)

*Public vocabulary traits*
    you could define equivalent traits in your own code. But they serve the important goal of setting down conventional solutions for common problems. These are especially valuable in public interfaces between crates and modules: by reducing needless variation, they make interfaces easier to understand, but they also increase the likelihood that features from different crates can simply be plugged together directly, without boilerplate or custom glue code. (**Default**, the reference-borrowing traits **AsRef, AsMut, Borrow, BorrowMut**, the fallible conversion traits **TryFrom, TryInto**, **ToOwned**, **Clone**)


## Drop

When a value's owner goes away, we say that Rust *drops* the value. 

When a value is dropped, if it implements std::ops::Drop, Rust calls its *drop* method, before proceeding to drop whatever values its fields or elements own, as it normally would. This implicit invocation of drop is the only way to call that method;

If a type implements *Drop*, it cannot implement the *Copy* trait. If a type is *Copy*, that means that simple byte-for-byte duplication is sufficient to produce an independent copy of the value.


## Sized

Rust can't store unsized values in variables or pass them as arguments. You can only deal with them through pointers like *&str* or *Box<dyn Write>*, which themselves are sized.


## Clone

## Copy

## Deref and DereMut

## Default

## AsRef and AsMut

## Borrow and BorrowMut

## From and Into

## TryFrom and TryInto

## ToOwned

## Borrow and ToOwned at Work: The Humble Cow



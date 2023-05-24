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

Since unsized types are so limited, most generic type variables should be restricted to *Sized* types. In fact, this is necessary so often that it is the implicit default in Rust: if you write *struct S<T> { ... }*, Rust understands you to mean *struct S<T: Sized> { ... }*. If you do not want to constrain *T* this way, you must explicitly opt out, writing *struct S<T: ?Sized> { ... }*. This *?Sized* syntax is specific to this case and means "not necessarily *Sized*."

When a type variable has the *?Sized* bound, people often say it is *questionably sized*: it might be *Sized*, or it might not.

A struct type's last field (but only its last) may be unsized, and such a struct is itself unsized.

You can't build an *RcBox<dyn Display>* value directly. Instead, you first need to create an ordinary, sized *RcBox* whose *value* type implements *Display*, like *RcBox<String>*. Rust then lets you convert a reference *&RcBox<String>* to a fat reference *&RcBox<dyn Display>*:

    let boxed_lunch: RcBox<String> = RcBox {
        ref_count: 1,
        value: "lunch".to_string()
    };

    use std::fmt::Display;
    let boxed_displayable: &RcBox<dyn Display> = &boxed_lunch;

This conversion happens implicitly when passing values to functions, so you can pass an &RcBox<String>* to a function that expects an *&RcBox<dyn Display>*:

    fn display(boxed: &RcBox<dyn Display>) {
        println!("For your enjoyment: {}", &boxed.value);
    }
    
    display(&boxed_lunch);

## Clone

The *clone* method should construct an independent copy of *self* and return it. Since this method's return type is *Self* and functions may not return unsized values, the *Clone* trait itself extends the *Sized* trait: this has the effect of bounding implementation's *Self* types to be *Sized*.

The *clone_from* method modifies *self* into a copy of source. The default definition of *clone_from* simply clones source and then moves that into *\*self*.

## Copy

simple types that don't own any resources can be *Copy* types, where assignment makes a copy of the source, rather than moving the value and leaving the source uninitialized.

a type is *Copy* if it implements the *std::marker::Copy* marker trait, which is defined as follows:

    trait Copy: clone {}

But because *Copy* is a marker trait with special meaning to the language, Rust permits a type to implement *Copy* only if a shallow byte-for-byte copy is all it needs.

Any type that implements the *Drop* trait cannot be *Copy*.

## Deref and DereMut

The traits are defined like this:

    trait Deref {
        type Target: ?Sized;
        fn deref(&self) -> &Self::Target;
    }
    
    trait DerefMut: Deref {
        fn deref_mut(&mut self) -> &mut Self::Target;
    }

The *deref* and *deref_mut* methods take a *&Self* reference and return a *&Self::Target* reference. *Target* should be something that *Self* contains, owns, or refers to: for *Box<Complex>* the *Target* type is *Complex*. Note that *DerefMut* extends *Deref*: if you can dereference something and modify it, certainly you should be able to borrow a shared reference to it as well. Since the methods return a reference with the same lifetime as
*self*, *self* remains borrowed for as long as the returned reference lives.

Since *deref* takes a *&Self* reference and returns a *&Self::Target* reference, Rust uses this to automatically convert references of the former type into the latter.(*&Self* => *&Self::Target*) In other words, if inserting a *deref* call would prevent a type mismatch, Rust inserts one for you. Implementing *DerefMut* enables the corresponding conversion for mutable references. These are called the **deref coercions**: one type is being
"coerced" into behaving as another.



## Default

## AsRef and AsMut

## Borrow and BorrowMut

## From and Into

## TryFrom and TryInto

## ToOwned

## Borrow and ToOwned at Work: The Humble Cow



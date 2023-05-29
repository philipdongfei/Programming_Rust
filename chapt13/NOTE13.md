# Utility Traits

*Language extension traits*
    Just as the operator overloading traits we covered in the previous chapter make it possible for you to use Rust's expression operators on your own types, there are several other standard library traits that serve as Rust extension points, allowing you to integrate your own types more closely with the language.
(**Drop, Deref, DerefMut**, and the conversion traits **From** and **Into**)

*Marker traits*
    These are traits mostly used to bound generic type variables to express constraints you can't capture otherwise.(**Sized, Copy**)

*Public vocabulary traits*
    you could define equivalent traits in your own code. But they serve the important goal of setting down conventional solutions for common problems. These are especially valuable in public interfaces between crates and modules: by reducing needless variation, they make interfaces easier to understand, but they also increase the likelihood that features from different crates can simply be plugged together directly, without boilerplate or custom glue code. (**Default**, the reference-borrowing traits **AsRef, AsMut, Borrow, BorrowMut**, the fallible conversion traits **TryFrom, TryInto**, **ToOwned**, **Clone**)


## Drop

    trait Drop { 
        fn drop(&mutself); 
    }

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

    trait Clone: Sized {
        fn clone(&self) -> Self;
        fn clone_from(&mut self, source: &Self) {
            *self = source.clone()
        }
    }

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

Since *deref* takes a *&Self* reference and returns a *&Self::Target* reference, Rust uses this to automatically convert references of the former type into the latter.(*&Self* => *&Self::Target*) In other words, if inserting a *deref* call would prevent a type mismatch, Rust inserts one for you. Implementing *DerefMut* enables the corresponding conversion for mutable references. These are called the **deref coercions**: one type is being "coerced" into behaving as another.

- If you have some *Rc<String>* value r and want to apply *String::find* to it, you can simply write *r.find('?')*, instead of *(*r).find('?')*: the method call implicitly borrows r and *&Rc<String>* coerces to *&String*, because *Rc<T>* implements *Deref<Target=T>*.
- You can use methods like *split_at* on *String* values, even though *split_at* is a method of the *str* slice type, because *String* implements *Deref<Target=str>*. There's no need for *String* to reiimplement all of *str's* methods, since you can coerce a *&str* from a *String*.
- If you have a vector of bytes *v* and you want to pass it to a function that expects a byte slice *&[u8]*, you can simply pass *&v* as the argument, since *Vec<T>* implements *Deref<Target=[T]>*.

Rust will apply serveral deref coercions in succession if necessary.

The deref coercions come with a caveat that can cause some confusion: Rust applies them to resolve type conflicts, but not to satisfy bounds on type variables.


## Default

    trait Default {
        fn default() -> Self;
    }

Some types have a reasonably obvious default value: the default vector or string is empty, the default number is zero, the default *Option* is *None*, and so on.

Rust does not implicitly implement *Default* for struct types, but if all of a struct's fields implement *Default*, you can implement *Default* for the struct automatically using *#[derive(Default)]*.


## AsRef and AsMut

When a type implements *AsRef<T>*, that means you can borrow a *&T* from it efficiently. *AsMut* is the analogue for mutable references. 

    trait AsRef<T: ?Sized> {
        fn as_ref(&self) -> &T;
    }
    
    trait AsMut<T: ?Sized> {
        fn as_mut(&mut self) -> &mut T;
    }

    impl<'a, T, U> AsRef<U> for &'a T
        where T: AsRef<U>,
              T: ?Sized, U: ?Sized
    {
        fn as_ref(&self) -> &U {
            (*self).as_ref()
        }            
    }

In other words, for any types *T* and *U*, if *T: AsRef<U>*, then *&T: AsRef<U>* as well: simply follow the reference and proceed as before. In particular, since *str: AsRef<Path>*, then *&str: AsRef<Path>* as well. In a sense, this is a way to get a limited form of deref coercion in checking *AsRef* bounds on type variables.

You might assume that if a type implements *AsRef<T>*, it should also implement *AsMut<T>*.


## Borrow and BorrowMut

The *std::borrow::Borrow* trait is similar to *AsRef*: if a type implements *Borrow<T>*, then its *borrow* method efficiently borrows a *&T* from it. But *Borrow* imposes more restrictions: a type should implement *Borrow<T>* only when a *&T* hashes and compares the same way as the value it's borrowed from.

    trait Borrow<Borrowed: ?Sized> {
        fn borrow(&self) -> &Borrowed;
    }

    trait BorrowMut<Borrowed: ?Sized>: Borrow<Borrowed> {
        fn borrow_mut(&mut self) -> &mut Borrowed;
    }

As a convenience, every *&mut T* type also implements *Borrow<T>*, returning a shared reference *&T* as usual.


## From and Into

The *std::convert::From* and *std::convert::Into* traits represent conversions that consume a value of one type and return a value of another.Whereas the *AsRef* and *AsMut* traits borrow a reference of one type from another, *From* and *Into* take ownership of their argument, transform it, and then return ownership of the result back to the caller.

    trait Into<T>: Sized {
        fn into(self) -> T;
    }
    
    trait From<T>: Sized {
        fn from(other: T) -> Self;
    }

You generally use *Into* to make your functions more flexible in the arguments they accept.

The *from* method serves as a generic constructor for producing an instance of a type from some other single value.

Given an appropriate *From* implementation, the standard library automatically implements the corresponding *Into* trait.



## TryFrom and TryInto

    pub trait TryFrom<T>: Sized {
        type Error;
        fn try_from(value: T) -> Result<Self, Self::Error>;
    }
    
    pub trait TryInto<T>: Sized {
        type Error;
        fn try_into(self) -> Result<T, Self::Error>;
    }

Where *From* and *Into* relate types with simple conversions, *TryFrom* and *TryInto* extend the simplicity of *From* and *Into* conversions with the expressive error handling afforded by *Result*.


## ToOwned

The *std::borrow::ToOwned* trait provides a slightly looser way to convert a reference to an owned value:

    trait ToOwned {
        type Owned: Borrow<Self>;
        fn to_owned(&self) -> Self::Owned;
    }

Unlike *clone*, which must return exactly *Self*, *to_owned* can return anything you could borrow a *&Self* from: the *Owned* type must implement *Borrow<Self>*.

## Borrow and ToOwned at Work: The Humble Cow

    enum Cow<'a, B: ?Sized>
        where B: ToOwned
    {
            Borrowed(&'a B),
            Owned(B as ToOwned>::Owned),
    }

A *Cow<B>* either borrows a shared reference to a B or owns a value from which we could borrow such a reference.Since *Cow* implements *Deref*, you can call methods on it as if it were a shared reference to a B: if it's Owned, it borrows a shared reference to the owned value; and if it's Borrowed, it just hands out the reference it's holding.

<font color="red">[6 things you can do with the Cow in Rust](https://dev.to/kgrech/6-things-you-can-do-with-the-cow-in-rust-4l55)</fond>




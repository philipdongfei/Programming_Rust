# Traits and Generics
Generics and traits are closely related: generic functions use traits in bounds to spell out what types of arguments they can be applied to.

## Using Traits
There is one unusual rule about trait methods: the trait itself must be in scope. Otherwise, all its methods are hidden.

### Trait Objects

    use std::io::Write;

    let mut buf: Vec<u8> = vec![];
    let writer: dyn Write = buf; // error: `Write` does not have a constant size


A variable's size has to be known at compile time, and types that implement *Write* can be any size.

What we want in Rust is the same thing, but in Rust, references are explicit:

    let mut buf: Vec<u8> = vec![]
    let writer: &mut dyn Write = &mut buf; // ok

A reference to a trait type, like *writer*, is called a *trait object*. Like any other reference, a trait object points to some value, it has a lifetime, and it can be either mut or shared.

### Trait object layout
In memory, a trait object is a fat pointer consisting of a pointer to the value, plus a pointer to a table representing that value's type. Each trait object therefore takes up two machine words, as shown in Figure 11-1.
Rust automatically converts ordinary references into trait objects when needed.

    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    let w: Box<dyn Write> = Box::new(local_file);

*Box<dyn Write>*, like *&mut dyn Write*, is a fat pointer: it contains the address of the writer itself and the address of the vtable.
This kind of conversion is the only way to create a trait object.

### Which to Use

**Trait Objects Advantages**
1. Trait objects are the right choice whenever you need a collection of values of mixed types, all together.
2. Another possible reason to use trait objects is to reduce the total amount of compiled code.

**Generics Advantages**
1. The first advantage is speed.
2. The second advantage of generics is that not every trait can support trait objects.
3. The third advantage of generics is that it's easy to bound a generic type parameter with serveral traits at once, as our *top_ten* function did when it required its *T* parameter to implement *Debug + Hash + Eq*.

### Default Methods

    /// A Writer that ignores whatever data you write to it.
    pub struct Sink
    
    use std::io::{Write, Result};
    
    impl Write for Sink {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            // Claim to have successfully written the whole buffer.
            Ok(buf.len())
        }
    
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

Why does Rust let us *impl Write for Sink* without defining this method? The answer is that the standard library's definition of the *Write* trait contains a *default implementation* for *write_all*

    trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;
    
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            let mut bytes_written = 0;
            while bytes_written < buf.len() {
            bytes_written += self.write(&buf[bytes_written..])?;
            }
            Ok(())
        }
        ...
    }



The *write* and *flush* methods are the basic methods that every writer must implement. A writer may also implement write_all, buf if not, the default implementation shown earlier will be used.

### Traits and Other People's Types

    use std::io::{self, Write};
    
    /// Trait for values to which you can send HTML.
    trait WriteHtml {
        fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
    }
    
    /// You can write HTML to any std::io writer.
    impl<W: Write> WriteHtml for W {
        fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
        ...
        }
    }


The line *impl<W: Write> WriteHtml for W* means "for every type *W* that implements *Write*, here's an implementation for *WriteHtml for W*."

We said earlier that when you implement a trait, either the trait or the type must be new in the current crate. This is called the *orphan rule*. It helps Rust ensure that trait implementations are **unique**. Your code can't *imple Write(**trait**) for u8(**type**)*, because both *Write* and *u8* are defined in the standard library.

### Self in Traits
    
    // error: the trait `Spliceable` cannot be made into an object
    fn splice_anything(left: &dyn Spliceable, right: &dyn Spliceable) {
        let combo = left.splice(right);
        // ...
    }

The reason is something we'll see again and again as we dig into the advanced features of traits. Rust rejects this code because it has no way to type-check the call *left.splice(right)*. The whole point of trait objects is that the type isn't known until run time. Rust has no way to know at compile time if *left* and *right* will be the same type, as required.

    pub trait MegaSpliceable {
        fn splice(&self, other: &dyn MegaSpliceable) -> Box<dyn MegaSpliceable>;        
    }

This trait is compatible with trait objects. There's no problem type-checking calls to this *.splice()* method because the type of the argument *other* is not required to match the type of *self*, as long as both types are *MegaSpliceable*.

### Subtraits
    
    /// Someone in the game world, either the player or some other
    /// pixie, gargoyle, squirrel, ogre, etc.
    trait Creature: Visible {
        fn position(&self) -> (i32, i32);
        fn facing(&self) -> Direction;
    }

The phrase *trait Creature: Visible* means that all creatures are visible. Every type that implements *Creature* must also implement the *Visible* trait:
    
    impl Visible for Broom {
        ...
    }

    impl Creature for Broom {
        ...
    }

In fact, Rust's subtraits are really just a shorthand for a bound on *Self*. A definition of *Creature* like this is exactly equivalent to the one shown earlier:

    trait Creature where Self: Visible {
        ...
    }

### Type-Associated Functions

## Fully Qualified Method Calls

    "hello".to_string()
    str::to_String("hello")
    ToString::to_string("hello")
    <str as ToString>::to_string("hello")

All four of these method calls do exactly the same thing. Most often, you'll just write *value.method()*. The other forms are *qualified* method calls. They specify the type or trait that a method is associated with. The last form, with the angle brackets, specifies both: a *fully qualified* method call.
 
* When two methods have the same name.
    
        outlaw.draw(); // error: draw on screen or draw pistol?

        Visible::draw(&outlaw); // ok: draw on screen
        HasPistol::draw(&outlaw); // ok: corral

* When the type of the *self* argument can't be inferred:

        let zero = 0; // type unspecified; could be `i8`, `u8`, ...

        zero.abs(); // error: can't call method `abs`
                    // on ambiguous numeric type
        
        i64::abs(zero); // ok

* When using the function itself as a function value:

        let words: Vec<String> = 
            line.split_whitespace() // iterator produces &str values
                .map(ToString::to_string) // ok
                .collect();

* When calling trait methods in macros.

### impl Trait

We could easily replace this hairy return type with a trait object:
    
    fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> Box<dyn Iterator<Item=u8>> {
        Box::new(v.into_iter().chain(u.into_iter()).cycle())

    }

However, taking the overhead of dynamic dispatch and an unavoidable heap allocation every time this function is called just to avoid an ugly type signature doesn't seem like a good trade, in most cases.

Rust has a feature called *impl Trait* designed for precisely this situation. *impl Trait* allows us to "erase" the type of a return value, specifying only the trait or traits it  implements, without dynamic dispatch or a heap allocation:

    fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item=u8> { // impl trait
        v.into_iter().chain(u.into_iter()).cycle()
    }

Now, rather than specifying a particular nested type of iterator combinator structs, *cyclical_zip's* signature just states that it returns some kind of iterator over *u8*. The return type expresses the intent of the function, rather than its implementation details.

It's important to note that Rust doesn't allow trait methods to use *impl Trait* return values. Supporting this will require some improvements in the languages's type system. Until that work is done, only free functions and functions associated with specific types can use *impl Trait* returns.

*impl Trait* can also be used in functions that take generic arguments. For instance, consider this simple generic function:
    
    fn print<T: Display>(val: T) {
        println!("{}", val);
    }

It is identical to this version using *impl Trait*:

    fn print(val: impl Display) {
        println!("{}", val);
    }

There is one important exception. Using generics allows callers of the function to specify the type of the generic arguments, like **print::<i32>(42)**, while using *impl Trait* does not.

### Associated Consts

You can declare a trait with an associated constant using the same syntax as for a struct or enum:
    
    trait Greet {
        const GREETING: &'static str = "Hello";
        fn greet(&self) -> String;
    }

Like assocaited types and functions, you can declare them but not give them a value:

    trait Float {
        const ZERO: Self;
        const ONE: Self;
    }

Then, implementors of the trait can define these values:

    impl Float for f32 {
        const ZERO: f32 = 0.0;
        const ONE: f32 = 1.0;
    }

    
    impl Float for f64 {
        const ZERO: f64 = 0.0;
        const ONE: f64 = 1.0;
    }

This allows you to write generic code that uses these values:

    fn add_one<T: Float + AddOutput=T>>(value: T) -> T {
        value + T::ONE
    }

Note that associated constants can't be used with trait objects, since the compiler relies on type information about the implementation in order to pick the right value at compile time.

## Reverse-Engineering Bounds

What we've been doing here is reverse-engineering the bounds on *N*, using the compiler to guide and check our work.

One advantage of Rust's approach is forward compatibility of generic code.You can change the implementation of a public generic function or method, and if you didn't change the signature, you haven't broken any of its users.

Another advantage of bounds is that when you do get a compiler error, at least the compiler can tell you where the trouble is.

## Traits as a Foundation


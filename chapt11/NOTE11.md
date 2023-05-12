# Traits and Generics
Generics and traits are closely related: generic functions use traits in bounds to spell out what types of arguments they can be applied to.

## Using Traits
There is one unusual rule about trait methods: the trait itself must be in scope. Otherwise, all its methods are hidden.

## Trait Objects

    use std::io::Write;

    let mut buf: Vec<u8> = vec![];
    let writer: dyn Write = buf; // error: `Write` does not have a constant size


A variable's size has to be known at compile time, and types that implement *Write* can be any size.

What we want in Rust is the same thing, but in Rust, references are explicit:

    let mut buf: Vec<u8> = vec![]
    let writer: &mut dyn Write = &mut buf; // ok

A reference to a trait type, like *writer*, is called a *trait object*. Like any other reference, a trait object points to some value, it has a lifetime, and it can be either mut or shared.

## Trait object layout
In memory, a trait object is a fat pointer consisting of a pointer to the value, plus a pointer to a table representing that value's type. Each trait object therefore takes up two machine words, as shown in Figure 11-1.
Rust automatically converts ordinary references into trait objects when needed.

    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    let w: Box<dyn Write> = Box::new(local_file);

*Box<dyn Write>*, like *&mut dyn Write*, is a fat pointer: it contains the address of the writer itself and the address of the vtable.
This kind of conversion is the only way to create a trait object.

## Which to Use

**Trait Objects Advantages**
1. Trait objects are the right choice whenever you need a collection of values of mixed types, all together.
2. Another possible reason to use trait objects is to reduce the total amount of compiled code.

**Generics Advantages**
1. The first advantage is speed.
2. The second advantage of generics is that not every trait can support trait objects.
3. The third advantage of generics is that it's easy to bound a generic type parameter with serveral traits at once, as our *top_ten* function did when it required its *T* parameter to implement *Debug + Hash + Eq*.

## Default Methods

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

## Traits and Other People's Types

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

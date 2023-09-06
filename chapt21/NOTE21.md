# Chapter 21. Macros

Rust supports **macros**, a way to extend the language in ways that go beyond what you can do with functions alone.
Macros are a kind of shorthand. During compilation, before types are checked and long before any machine code is generated, each macro call is *expanded*--that is, it's replaced with some Rust code.


## Macro Basics

A macro defined with **macro_rules!** works entirely by pattern matching. The body of a macro is just a series of rules:

    ( pattern1 ) => ( template1 );
    ( pattern2 ) => ( template2 );
    ...

Incidentally, you can use square brackets or curly braces instead of parentheses around the pattern or the template; it makes no difference to Rust. Likewise, when you call a macro, these are all equivalent:

    assert_eq!(gcd(6, 10), 2);
    assert_eq![gcd(6, 10), 2];
    assert_eq!{gcd(6, 10), 2}

The only difference is that semicolons are usually optional after curly braces. By convention, we use parentheses when calling **assert_eq!**, square brackets for **vec!**, and curly braces for **macro_rules!**.


### Basics of Macro Expansion



### Unintended Consequences

### Repetition

## Built-In Macros

## Debugging Macros

## Building the json! Macro

### Fragment Types

### Recursion in Macros

### Using Traits with Macros

### Scoping and Hygiene

### Importing and Exporting Macros


## Avoiding Syntax Errors During Matching

## Beyond macro_rultes!

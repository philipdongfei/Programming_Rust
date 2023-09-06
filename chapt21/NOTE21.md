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

Rust expands macros very early during compilation. The compiler reads your source code from beginning to end, defining and expanding macros as it goes. You can't call a macro before it is defined, because Rust expands each macro call before it even looks at the rest of the program.
Macro patterns are a mini-language within Rust. They're essentially regular expressions for matching code. But where regular expressions operate on characters, patterns operate on *tokens*--the numbers , names, punctuation marks, and so forth that are the building blocks of Rust programs. This means you can use comments and whitespace freely in macro patterns to make them as readable as possible. Comments and whitespace aren't tokens, so they don't affect matching.
Another important difference between regular expressions and macro patterns is that parentheses, brackets, and braces always occur in matched pairs in Rust. This is checked before macros are expanded, not only in macro patterns but throughout the language.
Macro templates aren't much different from any of a dozen template languages commonly used in web programming. The only difference--and it's a significant one--is that the output is Rust code.


### Unintended Consequences


### Repetition

    // 1. let buffer = vec![0_u8; 1000];
    // 2. let numbers = vec!["udon", "ramen", "soba"];
    // 3. let numbers = vec!["udon", , ];
    macro_rules! vec {
        ($elem:expr ; $n:expr) => {
            ::std::vec::from_elem($elem, $n)
        };
        ( $( $x:expr  ),* ) => {
            <[_]>::into_vec(Box::new([ $( $x ),* ]))
        };
        ( $( $x:expr ),+ , ) => {
            vec![ $( $x ),* ]
        };
    }

The code fragment **$x** is not just a single expression but a list of expressions. 
The first bit, <[_]>, is an unusual way to write the type "slice of something," while expecting Rust to infer the element type. Types whose names are plain identifiers can be used in expressions without any fuss, but types like **fn()**, **&str**, or **[_]** must be wrapped in angle brackets.
Unlike the rest of Rust, patterns using **$( ... ),*** do not automatically support an optional trailing comma. However, there's a standard trick for supporting trailing commas by adding an extra rule. That is what the third rule of our **vec!** macro does:

    ( $( $x:expr ),+ , ) => { // if trailing comma is present,
        vec![ $( $x ),* ]     // retry without it
    };

We use **$( ... ),+ ,** to match a list with an extra comma. Then, in the template, we call **vec!** recursively, leaving the extra comma out. This time the second rule will match.


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

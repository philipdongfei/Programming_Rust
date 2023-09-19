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

Here are three tools to help troubleshoot macros. (These features are all unstable, but since they're really designed to be used during development, not in code that you'd check in, that isn't a big problem in practice.)
First and simplest, you can ask **rustc** to show what your code looks like after expanding all macros. Use **cargo build --verbose** to see how Cargo is invoking **rustc**. Copy the **rustc** command line and **-Z unstable-options --pretty expanded** as options. The fully expanded code is dumped to your terminal. Unfortunately, this works only if your code is free of syntax errors.
Second, Rust provides a **log_syntax!()** macro that simply prints its arguments to the terminal at compile time. You can use this for **println!-**style debugging. This macro requiresthe **#![feature(log_syntax)]** feature flag.
Third, you can ask the Rust compiler to log all macro calls to the terminal. Insert **trace_macros!(true)**; somewhere in your code. From that point on, each time Rust expands a macro, it will print the macro name and arguments. 

## Building the json! Macro

### Fragment Types

The last two, **ident** and **tt**, support matching macro arguments that don't look like Rust code. **ident** matches any identifier. **tt** matches a single *token tree*: either a properly matched pair of brackets, (...),[...], or {...}, and everything in between, including nested token trees, or a single token that isn't a bracket, like 1926 or "Knots".

Token trees are exactly what we need for our **json!** macro. Every JSON value is a single token tree: numbers, strings, Boolean values, and **null** are all single tokens; objects and arrays are bracketed. So we can write the patterns like this:

    macro_rules! json {
        (null) => {
            Json::Null    
        };
        ([ $( $element:tt ),* ]) => {
            Json::Array(...)
        };
        ({ $( $key:tt : $value:tt ),* }) => {
            Json::Object(...)
        };
        ($other:tt) => {
            ... // TODO: Return Number, String, or Boolean
        };
    }



### Recursion in Macros

We need to convert each element of the array from JSON form to Rust.

    ([ $( $element:tt ),* ]) => {
        Json::Array(vec![ $( json!($element) ),* ])
    };

Objects can be supported in the same way:

    ({ $( $key:tt : $value:tt ),* }) => {
        Json::Object(Box::new(vec![
            $( ($key.to_string(), json!($value)) ),*
        ].into_iter().collect()))    
    };

The compiler imposes a recursion limit on macros: 64 calls, by default. That's more than enough for normal uses of **json!**, but complex recursive macros sometimes hit the limit. You can adjust it by adding this attrribute at the top of the crate where the macro is used:

    #![recursion_limit = "256"]



### Using Traits with Macros

    macro_rules! impl_from_num_for_json {
        ( $( $t:ident )* ) => {
            $(
                impl From<$t> for Json {
                    fn from(n: $t) -> Json {
                        Json::Number(n as f64)
                    }
                }
            )*
        };    
    }

    impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128
                        usize isize f32 f64);

### Scoping and Hygiene

A surprisingly tricky aspect of writing macros is that they involve pasting code from different scopes together. So the next few pages cover the two ways Rust handles scoping: one way for local variables and arguments, and another way for everything else.
To show why this matters, let's rewrite ourrule for parsing JSON objects to eliminate the temporary vector.

    
    ({ $( $key:tt : $value:tt ),* }) => {
        {
            // Scoping and Hygiene 
            let mut fields = Box::new(HashMap::new());
            $( fields.insert($key.to_string(), json!($value)); )*
            Json::Object(fields)

        }
    };

Now we're populating the **HashMap** not by using **collect()** but by repeatedly calling the **.insert()** method. This means we need to store the map in a temporary variable, which we've called **fields**.

But then what happens if the code that calls **json!** happens to use a variable of its own, also named **fields**?

    let fields = "Fieldds, W.C.";
    let role = json!({
        "name": "Larson E. Whipsnade",
        "actor": fields
    }); 

Expanding the macro would paste together two bits of code, both using the name **fields** for different things!

    let fields = "Fields, W.C.";
    let role = {
        let mut fields = Box::new(HashMap::new());
        fields.insert("name".to_string(), Json::from("Larson E. Whipsnade"));
        fields.insert("actor".to_string(), Json::from(fields));
        Json::Object(fields)
    };

This may seem like an unavoidable pitfall whenever macros use temporary variables, and you may already be thinking through the possible fixes. 
The surprise here is that *the macro works as is*. Rust renames the variable for you! This feature, first implemented in Scheme macros, is called *hygiene*, and so Rust is said to have *hygienic macros*.
The easiest way to understand macro hygiene is to imagine that every time a macro is expanded, the parts of the expansion that come from the macro itself are painted a different color.
Variables of different colors, then, are treated as if they had different names:

    let fields = "Fields, W.C.";
    let role = <font color="orange">{
        let mut fields = Box::new(HashMap::new());
        fields.insert(<font color="black">"name"</font>.to_string(), <font color="blue">Json::from</font>("Larson E. Whipsnade"));
        fields.insert(<font color="black">"actor"</font>.to_string(), <font color="cyan">Json::from</font>(fields));
        Json::Object(fields)
    }</font>;

Note that bits of code that were passed in by the macro caller and pasted into the output, such as "name" and "actor", keep their original color(black). Only tokens that originate from the macro template are painted.
Now there's one variable named **fields** (declared in the caller) and a separate variable named <font color="orange">fields</font> (introduced by the macro). Since the names are different colors, the two variables don't get confused.
If a macro really does need to refer to a variable in the caller's scope, the caller has to pass the name of the variable to the macro.
(The paint metaphor isn't meant to be an exact description of how hygiene works. The real mechanism is event a little smarter than that, recognizing two identifiers as the same, regardless of "paint", if they refer to a common variable that's in scope for both the macro and its caller. But cases like this are rare in Rust. If you understand the preceding example, you know enough to use hygienic macros.)
That's because hygiene in Rust is limited to local variables and arguments. When it comes to constants, types, methods, modules, statics, and macro names, Rust is "colorblind".

    macro_rules! setup_req {
        ($req:ident, $server_socket:ident) => {
            let $req = ServerRequest::new($server_socket.session());
        }    
    }

    fn handle_http_request(server_socket: &ServerSocket) {
        setup_req!(req, server_socket);
        ... // code that uses `req`
    }

Hygiene makes this macro a little wordier to use, but that's a feature, not a bug: it's easier to reason about hygienic macros knowing that they can't mess with local variables behind your back. If you search for an identifier like **server_socket** in a function, you'll find all the places where it's used, including macro calls.


### Importing and Exporting Macros

Macros that are visible in one module are automatically visible in its child modules. To export macros from a module "upward" to its parent module, use the **#[macro_use]** attribute.
Macros marked with **#[macro_export]** are automatically pub and can be referred to by path, like other items.
Instead, the macro should use absolute paths to any names it uses. **macro_rules!** provides the special fragment **$crate** to help with this. This is not the same as **crate**, which is a keyword that can be used in paths anywhere, not just in macros. **$crate** acts like an absolute path to the root module of the crate where the macro was defined.



## Avoiding Syntax Errors During Matching

First, avoid confusable rules.
The other way to avoid spurious syntax errors is by putting more specific rules first. 

## Beyond macro_rultes!

[The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/), by Daniel Keep et al., is an excellent handbook of advanced **macro_rules!** programming. 
What makes a procedural macro "procedural" is that it's implemented as a Rust function, not a declarative rule set. This function interacts with the compiler through a thin layer of abstraction and can be arbitrarily complex.
Because procedural macros interact with compiler internals, writing effective macros requires an understanding of how the compiler operates that is out of the scope of this book. It is, however, extensively covered in the [online documentation](https://oreil.ly/0xB2x).
Perhaps, having read all this, you've decided that you hate macros. What then? An alternative is to generate Rust code using a build script. The [Cargo documentation](https://doc.rust-lang.org/cargo/reference/build-scripts.html) shows how to do it step by step.

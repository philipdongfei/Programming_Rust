# Enums and Patterns
## Enums
Casting a C-style enum to an integer is allowed:
'''
assert_eq!(HttpStatus::Ok as i32, 200);
'''
However, casting in the other direction, from the integer to the enum, is not. Unlike C and C++, Rust guarantees that an enum value is only ever one of the values spelled out in the enum declaration. An unchecked cast from an integer type to an enum type could break this guarantee, so it's not allowed.

## Enums with Data
In all, Rust has three kinds of enum variant, echoing the three kinds of struct we showed in the previous chapter. Variants with no data correspond to unit-like structs. Tuple variants look and function just like tuple structs. Struct variants have curly braces and named fields. A single enum can have variants of all three kinds:
'''
enum RelationshipStatus {
    Single, // unit-like structs
    InARelationship,
    ItsComplicated(Option<String>), // tuple variants
    ItsExtremelyComplicated {
        car: DifferentialEquation,
        cdr: EarlyModernistPoem,
    }, // struct variants
}
'''

## Rich Data Structures Using Enums
'''
use std::collections::HashMap;
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),   // 3 words + 1 tag byte
    Array(Vec<Json>), // 3 words + 1 tag byte
    Object(Box<HashMap<String, Json>>),
}
'''
A HashMap is larger still. If we had to leave room for it every *Json* value, they would be quite large, eight words or so. But a *Box<HashMap>* is a single word: it's just a pointer to heap-allocated data.

## Generic Enums
One unobvious detail is that Rust can eliminate the tag field of Option<T> when the type T is a reference, Box, or other smart pointer type. Since none of those pointer types is allowed to be zero, Rust represent *Option<Box<i32>>, say, as a single machine word: 0 for None and nonzero for *Some* pointer. This makes such *Option* types close analogues to C or C++ pointer values that could be null. The difference is that Rust's type system requires you to check
that an *Option* is *Some* before you can use its contents. This effectively eliminates null pointer dereferences.


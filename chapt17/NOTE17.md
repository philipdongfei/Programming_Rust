# Strings and Text

## Some Unicode Background

### ASCII, Latin-1, and Unicode


### UTF-8

* Since UTF-8 encodes code points 0 through 0x7f as nothing more than the bytes 0 through 0x7f, a range of bytes holding ASCII text is valid UTF-8.
* From looking at any byte's upper bits, you can immediately tell whether it is the start of some character's UTF-8 encoding or a byte from the midst of one.
* An encoding's first byte alone tells you the encoding's full length, via its leading bits.
* Since no encoding is longer than four bytes, UTF-8 processing never requires unbounded loops, which is nice when working with untrusted data.
* In well-formed UTF-8, you can always tell unambiguously where characters' encodings begin and end, even if you start from an arbitrary point in the midst of the bytes.

### Text Directionality

Whereas scripts like Latin, Cyrillic, and Thai are written from left to rigth, other scripts like Hebrew and Arabic are written from right to left.


## Characters(char)

A Rust **char** is a 32-bit value holding a Unicode code point. A **char** is guaranteed to fall in the range from 0 to 0xd7ff or in the range 0xe000 to 0x10ffff;

### Classifying Characters

The **char** type has methods for classifying characters into a few common categories.

### Handling Digits

### Case Conversion for Characters

* **ch.is_lowercase(), ch.is_uppercase()**
* **ch.to_lowercase(), ch.to_uppercase()**
    Return iterators that produce the characters of the lower- and uppercase equivalents of ch, according to the Unicode Default Case Conversion algorithms
    These methods return an iterator instead of a single character because case conversion in Unicode isn't always a one-to-one process.
    

### Case Conversion for Integers

Rust's as operator will convert a char to any integer type, silently masking off any upper bits.
The as operator will convert any u8 value to a char, and char implements **From<u8>** as well, but wider integer types can represent invalid code points, so for those you must use **std::char::from_u32**, which returns **Option<char>**.

## String and str

## Formatting Values

## Regular Expressions

## Normalization



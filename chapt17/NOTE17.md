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

Rust's **String** and **str** types are guaranteed to hold only well-formed UTF-8. The library ensures this by restricting the ways you can create **String** and **str** values and the operations you can perform on them, such that the values are well-formed when introduced and remain so as you work with them. All their methods protect this guarantee: no safe operation on them can introduce ill-formed UTF-8.
Rust places text-handling methods on either **str** or **String** depending on whether the method needs a resizable buffer or is content just to use the text in place.
A **String** is implemented as a wrapper around a **Vec<u8>** that ensures the vector's contents are always well-formed UTF-8.

### Creating String Values

There are a few common ways to create **String** values:

* **String::new()**
* **String::with_capacity(n)**
* **str_slice.to_string()**
* **iter.collect()**
* **slice.to_owned()**
    Returns a copy of slice as a freshly allocated **String**. The **str** type cannot implement **Clone**: the trait would require **clone** on a **&str** to return a **str** value, but **str** is unsized.

### Simple inspection

These methods get basic information from string slices:

* **slice.len()**
* **slice.is_empty()**
* **slice[range]**
    Returns a slice borrowing the given portion of **slice**. Partially bounded and unbounded ranges are OK;
    Note that you cannot index a string slice with a single position, like **slice[i]**. Fetching a single character at a given byte offset is a bit clumsy: you must produce a **chars** iterator over the slice, and ask it to parse one character's UTF-8.
* **slice.split_at(i)**
* **slice.is_char_boundary(i)**

### Appending and Inserting Text

The following methods add text to a **String**:

* **string.push(ch)**
* **string.push_str(slice)**
* **string.extend(iter)**
* **string.insert(i, ch)**
* **string.insert_str(i, slice)**
* String implements **std::fmt::Write**, meaning that the **write!** and **writeln!** macros can append formatted text to Strings;
* Since **String** implements **Add<&str>** and **AddAssign<&str>**, you can write code like this.

### Removing and Replacing Text

**String** has a few methods for removing text (these do not affect the string's capacity; use **shrink_to_fit** if you need to free memory):

* **string.clear()**
* **string.truncate(n)**
* **string.pop()**
* **string.remove(i)**
* **string.drain(range)**
* **string.replace_range(range, replacement)**

### Conventions for Searching and Iterating

Rust's standard library functions for searching text and iterating over text follow some naming conventions to make them easier to remember:

**r**
    Most operations process text from start to end, but operations with names starting with **r** work from end to start.

**n**
    Iterators with names ending in **n** limit themselves to a given number of matches.

**_indices**
    Iterators with names ending in **_indices** produce, together with their usual iteration values, the byte offsets in the slice at which they appear.

### Patterns for Searching Text

The standard library supports four main kinds of patterns:

* A **char** as a pattern matches that character.
* A **String** or **&str** or **&&str** as a pattern matches a substring equal to the pattern.
* A **FnMut(char) -> bool** closure as a pattern matches a single character for which the closure returns true.
* A **&[char]** as a pattern (not a **&str**, but a slice of **char** values) matches any single character that appears in the list. Note that if you write out the list as an array literal, you may need to call as_ref() to go get the type right:
    
        let code = "\t      function noodle() {  ";
        // pub fn trim_start_matches<'a, P>(&'a self, pat: P) -> &'a str where P: Pattern<'a>, 
        // fn as_ref(&self) -> &str
        assert_eq!(code.trim_start_matches([' ', '\t'].as_ref()),
                    "function noodle() { ");
        // Shorter equivalent: &[' ', '\t'][..]

Otherwise, Rust will be confused by the fixed-size array type **&[char; 2]**, which is unfortunately not a pattern type.


### Searching and Replacing

Rust has a few methods for searching for patterns in slices and possibly replacing them with new text.

* **slice.contains(pattern)**
* **slice.starts_with(pattern), slice.ends_with(pattern)**
* slice.find(pattern), slice.rfind(pattern)**
* **slice.replace(pattern, replacement)**
* **slice.replacen(pattern, replacement, n)**


### Iterating over Text

The standard library provides several ways to iterate over a slice's text.

* **slice.chars()**
* **slice.char_indices()**
* **slice.bytes()**
* **slice.lines()**
* **slice.split(pattern)**
* **slice.rsplit(pattern)**
* **slice.split_terminator(pattern), slice.rsplit_terminator(pattern)**
* **slice.splitn(n, pattern), slice.rsplitn(n, pattern)**
* **slice.split_whitespace(), slice.split_ascii_whitespace()**
* **slice.matches(pattern)**
* **slice.match_indices(pattern), slice.rmatch_indices(pattern)**


### Trimming

To *trim* a string is to remove text, usually whitespace, from the beginning or end of the string.

* **slice.trim()**
* **slice.trim_matches(pattern)**

### Case Conversion for Strings

The methods **slice.to_uppercase()** and **slice.to_lowercase()** return a freshly allocated string holding the text of **slice** converted to uppercase or lowercase.

### Parsing Other Types from Strings

Rust provides standard traits for both parsing values from strings and producing textual representations of values.


### Converting Other Types to Strings

There are three main ways to convert nontextual values to strings:

* Types that have a natural human-readable printed form can implement the **std::fmt::Display** trait, which lets you use the {} format specifier in the **format!** macro. The smart pointer types **Box<T>**, **Rc<T>**, and **Arc<T>** implement **Display** if T itself does: Their displayed form is simply that of their referent.
* If a type implements **Display**, the standard library automatically implements the **std::str::ToString** trait for it, whose sole method **to_string** can be more convenient when you don't need the flexibility of **format!**.
* Every public type in the standard library implements **std::fmt::Debug**, which takes a value and formats it as a string in a way helpful to programmers. The easiest way to use **Debug** to produce a string is via the **format!** macro's {:?} format specifier.

### Borrowing as Other Text-Like Types

You can borrow a slice's contents in several different ways:

* Slices and Strings implement **AsRef<str>**, **AsRef<[u8]>**, **AsRef<Path>**, and **AsRef<OsStr>**.
* Slices and strings also implement the **std::borrow::Borrow<str>** trait.

### Accessing Text as UTF-8

There are two main ways to get at the bytes representing text, depending on whether you want to take ownership of the bytes or just borrow them:

* **slice.as_bytes()**
* **string.into_bytes()**

### Producing Text from UTF-8 Data

If you have a block of bytes that you believe contains UTF-8 data, you have a few options for converting them into **Strings** or slices, depending on how you want to handle errors:

* **str::from_utf8(byte_slice)**
* **String::from_utf8(vec)**
* **String::from_utf8_lossy(byte_slice)**
* **String::from_utf8_unchecked**
* **str::from_utf8_unchecked**

### Putting Off Allocation

This dynamic character is the hint to consider using **std::borrow::Cow**, the clone-on-write type that can hold either owned or borrowed data.
**Cow<'a, T>** isan enum with two variants: **Owned** and **Borrowed**. **Borrowed** holds a reference **&'a T**, and **Owned** holds the owning version of **&T: String** for **&str**, **Vec<i32>** for **&[i32]**, and so on. Whether **Owned** or **Borrowed**, a **Cow<'a, T>** can always produce a **&T** for you to use. 

Keep in mind that not every **Cow<..., str>** must be **'static**: you can **Cow** to borrow previously computed text until the moment a copy becomes necessary.

### Strings as Generic Collections

**String** implements both **std::default::Default** and **std::iter::Extend: default** returns an empty string, and **extend** can append characters, string slices, **Cow<..., str>**s, or strings to the end of a string.
The **&str** type also implements **Default**, returning an empty slice.

## Formatting Values

## Regular Expressions

## Normalization



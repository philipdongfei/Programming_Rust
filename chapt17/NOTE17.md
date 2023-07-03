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

Several standard library features share this little language for formatting strings:

* The **format!** macro uses it to build **String**s.
* The **println!** and **print!** macros write formatted text to the standard output stream.
* The **writeln!** and **write!** macros write it to a designated output stream.
* The **panic!** macro uses it to build an (ideally informative) expression of terminal dismay.

The template's {...} forms are called *format parameters* and have the form {*which:how*}. Both parts are optional; {} is frequently used.
The *which* value selects which argument following the template should take the parameter's place. You can select arguments by index or by name. Parameters with no *which* value are simply paired with arguments from left to right.
The *how* value says how the argument should be formatted: how much padding, to which precision, in which numeric radix, and so on. If *how* is present, the colon before it is required.


### Formatting Text Values

When formatting a textual type like **&str** or **String** (**char** is treated like a single character string), the *how* value of a parameter has serveral parts, all optional:

* A *text length limit*. Rust truncates your argument if it is longer than this. If you specify no limit, Rust uses the full text.
* A *minimum field width*. After any trucation, if your argument is shorter than this, Rust pads it on the right (by default) with spaces (by default) to make a field of this width. If ommitted, Rust donesn't pad your argument.
* An *alignment*. If your argument needs to be padded to meet the minimum field width, this says where your text should be placed within the field.
* A *padding* character to use in this padding process. If ommitted, Rust uses spaces. If you specify the padding character, you must also specify the alignment.

Rust's formatter has a naïve understanding of width: it assumes each character occupies one column, with no regard for combining characters, half-width katakana, zero-width spaces, or the other messy realities of Unicode.

Since filename paths are not necessarily well-formed UTF-8, **std::path::Path** isn't quite a textual type; you can't pass a **std::path::Path** directly to a formatting macro.


### Formatting Numbers

When the formatting argument has a numeric type like **usize** or **f64**, the parameter's *how* value has the following parts, all optional:

* A *padding* and *alignment*, which work as they do with textual types.
* A **+** character, requesting that the number's sign always be shown, even when the argument is positive.
* A **#** character, requesting an explicit radix prefix like **0x** or **0b**.
* A **0** character, requesting that the minimum field width be satisfied by including leading zeros in the number, instead of the usual padding approach.
* A *minimum field width*. If the formatted number is not at least this wide, Rust pads it on the left (by default) with spaces (by default) to make a field of the given width.
* A *precision* for floating-point arguments, indicating how many digits Rust should include after the decimal  point.
* A *notation*. For integer types, this can be **b** for binary, **o** for octal, or **x** or **X** for hexadecimal with lower- or uppercase letters. If you included the **#** character, these include an explicit Rust-style radix prefix, **0b**, **0o**, **0x**, or **0X**.

### Formatting Other Types

Beyond strings and numbers, you can format several other standard library types:

* Error types can all be formatted directly, making it easy to include them in error messages.
* You can format internet protocol address types like **std::net::IpAddr** and **std::net::SocketAddr**.
* The Boolean **true** and **false** values can be formatted, although these are usually not the best strings to present directly to end users.


### Formatting Values for Debugging

To help with debugging and logging, the {:?} parameter formats any public type in the Rust standard library in a way meant to be helpful to programmers.


### Formatting Pointers for Debugging

Normally, if you pass any sort of pointer to a formatting macro--a reference, a Box, an Rc--the macro simply follows the pointer and formats its referent; the pointer itself is not of interest. But when you're debugging, it's sometimes helpful to see the pointer: an address can serve as a rough "name" for an individual value, which can be illuminating when examining structures with cycles or sharing.

### Referring to Arguments by Index or Name

A format parameter can explicitly select which argument it uses.
You can also select arguments by name.
You can mix indexed, named, and positional (that is, no index or name) parameters together in a single formatting macro use. The positional parameters are paired with arguments from left to right as if the indexed and named parameters weren't there: 
Named arguments must appear at the end of the list.

### Dynamic Widths and Precisions

A parameter's minimum field width, text length limit, and numeric precision need not always be fixed values; you can choose them at run time.

But if you'd like to choose the field width at run time, you can write:

    format!("{:>1$}", content, get_width())

Writing **1$** for the minimum field width tells **format!** to use the value of the second argument as the width. The cited argument must be a **usize**. You can also refer to the argument by name:

    format!("{:>width$}", content, width=get_width())
    format!("{:>width$.limit$}", content,
        width=get_width(), limit=get_limit())

In place of the text length limit or floating-point precision, you can also write *, which says to take the next positional argument as the precision. The following clips **content** to at most **get_limit()** characters:

    format!("{:.*}", get_limit(), content)


### Formatting Your Own Types

The formatting traits all have the same structure, differing only in their names. We'll use **std::fmt::Display** as a representative:

    trait Display {
        fn fmt(&self, dest: &mut std::fmt::Formatter)
            -> std::fmt::Result;
    }

The **fmt** method's job is to produce a properly formatted representation of **self** and write its characters to **dest**. In addition to serving as an output stream, the **dest** argument also carries details parsed from the format parameter, like the alignment and minimum field width.

### Using the Formatting Language in Your Own Code

You can write your own functions and macros that accept format templates and arguments by using Rust's **format_args!** macro and the **std::fmt::Arguments** type. 

## Regular Expressions

The external *regex* crate is Rust's official regular expression library. It provides the usual searching and matching functions. It has good support for Unicode, but it can search byte strings as well.

### Basic Regex Use

A **Regex** value represents a parsed regular expression ready to use. The **Regex::new** constructor tries to parse a **&str** as a regular expression, and returns a **Result**.


### Building Regex Values Lazily

The **Regex::new** constructor can be expensive: constructing a **Regex** for a 1,200-character regular expression can take almost a milisecond on a fast developer machine, and even a trival expression takes microseconds.
The **lazy_static** crate provides a nice way to construct static values lazily the first time they are used.

## Normalization

Most users would consider the French word for tea, thé, to be three characters long. However, Unicode actually has two ways to represent this text:

* In the *composed* form, thé comprises the three characters t, h, and é, where é is a single Unicode character with code point 0xe9.
* In the *decomposed* form, thé comprises the four characters, t, h, e, and \u{301}, where the e is the plain ASCII character, without an accent, and code point 0x301 is the "combining acute accent" character, which adds an acute accent to whatever character it follows.

### Normalization Forms

Unicode defines four normalized forms, each of which is appropriate for different uses.

* First, do you prefer characters to be as *composed* as possible or as *decomposed* as possible?
The composed form generally has fewer compatibility problems, since it more closely matches the representations most languages used for their text before Unicode became established. It may also work better with naïve string formatting features like Rust's **format!** macro. The decomposed form, on the other hand, may be better for displaying text or searching, since it makes the detailed structure of the text more explicit.
* The second question is: if two character sequences represent the same fundamental text but differ in the way that text should be formatted, do you want to treat them as equivalent or keep them distinct?


Unicode Normalization Form C and Normalization Form D (NFC and NFD) use the maximally composed and maimally decomposed forms of each character, but do not try to unify compatibility equivalent sequences. The NFKC and NFKD Normalization forms are like NFC and NFD, but normalize all compatibility equivalent sequences to some simple representative of their class.

### The unicode-normalization Crate

Rust's **unicode-normalization** crate provides a trait that adds methods to **&str** to put the text in any of the four normalized forms.
Although any substring of a normalized string is itself normalized, the concatenation of two normalized strings is not necessarily normalized.
As long as a text uses no unassigned code points when it is normalized, Unicode promises that its normalized form will not change in future versions of the standard. This means that normalized forms are generally safe to use in persistent storage, even as the Unicode standard evolves.





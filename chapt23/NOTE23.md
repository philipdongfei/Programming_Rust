# Foreign Functions

There are many critical libraries and interfaces implemented in other languages that we would like to be able to use in our Rust programs. Rust's *foreign function interface* (FFI) lets Rust code call functions written in C, and in some cases C++. 

## Finding Common Data Representations

The common denominator of Rust and C is machine language, so in order to anticipate what Rust values look like to C code, or vice versa, you need to consider their machine-level representations.
To cope with this variability, Rust's **std::os::raw** module defines a set of Rust types that are guaranteed to have the same representation as certain C types.

| C type                    | Corresponding std::os::raw type   |
| :---                      | :---                              |
| short                     | c_short                           |
| int                       | c_int                             |
| long                      | c_long                            |
| long long                 | c_longlong                        |
| unsigned short            | c_ushort                          |
| unsigned, unsigned int    | c_ushort                          |
| unsigned long             | c_ulong                           |
| unsigned long long        | c_ulonglong                       |
| char                      | c_char                            |
| signed char               | c_schar                           |
| unsigned char             | c_uchar                           |
| float                     | c_float                           |
| double                    | c_double                          |
| void *, const void *      | *mut c_void, *const c_void        |

- Except for **c_void**, all the Rust types here are aliases for some primitive Rust type
- A Rust **bool** is equivalent to a C or C++ **bool**.
- Rust's 32-bit **char** type is not the analogue of **wchar_t**, whose width and encoding vary from one implementation to another.
- Rust's primitive **usize** and **isize** types have the same representations as C's **size_t** and **ptrdiff_t**.
- C and C++ pointers and C++ references correspond to Rust's raw pointer types, **\*mut T** and **\*const T**.
- Technically, the C standard permits implementations to use representations for which Rust has no corresponding type: 36-bit integers, sign-and-magnitude representations for signed values, and so on. In practice, on every platform Rust has been ported to, every common C integer type has a match in Rust.

For defining Rust struct types compatible with C structs, you can use the **\#\[repr\(C\)\]** attribute. Placing **\#\[repr\(C\)\]** above a struct definition asks Rust to lay out the struct's fields in memory the same way a C compiler would lay out the analogous C struct type.

You can define a Rust type with an identical representation as follows:

    use std::os::raw::{c_char, c_int};

    #[repr(C)]
    pub struct git_error {
        pub message: *const c_char,
        pub klass: c_int
    }

The **\#\[repr\(C\)\]** attribute affects only the layout of the struct itself, not the representations of its individual fields, so to match the C struct, each field must use the C-like type as well: **\*const c_char** for **char \***, **c_int** for **int**, and so on.
In this particular case, the **\#\[repr\(C\)\]** attribute probably doesn't change the layout of **git_error**. There really aren't too many interesting ways to lay out a pointer and an integer. But whereas C and C++ guarantee that a structure's members appear in memory in the order they're declared, each at a distinct address, Rust reorders fields to minimize the overall size of the struct, and zero-sized types take up no space. The **\#\[repr\(C\)\]** attribute tells
Rust to follow C's rules for the given type.
You can also use \#\[repr\(C\)\] to control the representation of C-style enums:

    #[repr(C)]
    #[allow(non_camel_case_types)]
    enum git_error_code {
        GIT_OK      = 0,
        GIT_ERROR   = -1,
        GIT_ENOTFOUND = -3,
        GIT_EEXISTS = -4,
        ...
    }

Normally, Rust plays all sorts of games when choosing how to represent enums. For example, we mentioned the trick Rust uses to store **Option\<\#T\>** in a single word (if **T** is sized). Without \#\[repr\(C\)\], Rust would use a single byte to represent the **git_error_code** enum; with \#\[repr\(C\)\], Rust uses a value the size of a **C int**, just as **C** would.
You can also ask Rust to give an enum the same representation as some integer type. Starting the preceding definition with \#\[repr\(i16\)\] would give you a 16-bit type with the same representation as the following C++ enum:

    #include <stdint.h>

    enum git_error_code: int16_t {
        GIT_OK      = 0,
        GIT_ERROR   = -1,
        GIT_ENOTFOUND = -3,
        GIT_EEXISTS = -4,
        ...
    };

As mentioned earlier, \#\[repr\(C\)\] applies to unions as well. Fields of \#\[repr\(C\)\] unions always start at the first bit of the union's memory--index 0.

Passing strings between Rust and C is a little harder. C represents a string as a pointer to an array of characters, terminated by a null character. Rust, on the other hand, stores the length of a string explicitly, either as a field of a **String** or as the second word of a fat reference **\&str**. Rust strings are not null-terminated; in fact, they may include null characters in their contents, like any other character.
This means that you can't borrow a Rust string as a C string: if you pass C code a pointer into a Rust string, it could mistake an embedded null character for the end of the string or run off the end looking for a terminating null that isn't there. Going the other direction, you may be able to borrow a C string as a Rust **\&str**, as long as its contents are well-formed UTF-8.


## Declaring Foreign Functions and Variables

An **extern** block declares functions or variables defined in some other library that the final Rust executable will be linked with. 

    use std::os::raw::c_char;

    extern {
        fn strlen(s: *const c_char) -> usize;
    }

This gives Rust the function's name and type, while leaving the definition to be linked in later.
Rust assumes that functions declared inside **extern** blocks use C conventions for passing arguments and accepting return values. They are defined as **unsafe** functions. These are the right choices for **strlen**: it is indeed a C function, and its specification in C requires that you pass it a valid pointer to a properly terminated string, which is a contract that Rust cannot enforce. (Almost any function that takes a raw pointer must be **unsafe**: safe Rust can
construct raw pointers from arbitrary integers, and dereferencing such a pointer would be undefined behavior.)


## Using Functions from Libraries

To use functions provided by a particular library, you can place a **\#\[link\]** attribute atop the **extern** block that names the library Rust should link the executable with.
this example: git-toy

    use std::os::raw::c_int;

    #[link(name = "git2")]
    extern {
        pub fn git_libgit2_init() -> c_int;
        pub fn git_libgit2_shutdown() -> c_int;
    }

    fn main() {
        unsafe {
            git_libgit2_init();
            git_libgit2_shutdown();
        }
    }

    $ cd /home/jimb/libgit2
    $ mkdir build
    $ cd build
    $ cmake ..
    $ cmake --build .

    $ cd /home/jimb
    $ cargo new --bin git-toy
    $ cd git-toy
    $ cargo run

    // To create your build script, add file named build.rs 
    // in the same directory as the Cargo.toml file, 
    // with the following contents:
    fn main() {
        println!(r"cargo:rustc-link-search=native=/home/jimb/libgit2/build");
    }

    $ export LD_LIBRARY_PATH=/home/jimb/libgit2/build:$LD_LIBRARY_PATH
    $ cargo run


## A Raw Interface to libgit2

## A Safe Interface to libgit2

## Conclusion



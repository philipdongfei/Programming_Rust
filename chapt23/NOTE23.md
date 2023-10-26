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

The program we'll write is very simple: it takes a path as a command-line argument, opens the Git repository there, and prints out the head commit. But this is enough to illustrate the key strategies for building safe and idiomatic Rust interfaces.
For the raw interface, the program will end up needing a somewhat larger collection of functions and types from **libgit2** than we used before, so it makes sense to move the **extern** block into its own module. We'll create a file named *raw.rs* in *git\-toy\/src* whose contents are as follows:


    #![allow(non_camel_case_types)]
    
    use std::os::raw::{c_int, c_char, c_uchar};
    
    #[link(name = "git2")]
    extern {
        pub fn git_libgit2_init() -> c_int;
        pub fn git_libgit2_shutdown() -> c_int;
        // libgit2(v1.7) giterr_last(book) -> git_error_last
        pub fn git_error_last() -> *const git_error;
    
        pub fn git_repository_open(out: *mut *mut git_repository,
                                    path: *const c_char) -> c_int;
        pub fn git_repository_free(repo: *mut git_repository);
    
        pub fn git_reference_name_to_id(out: *mut git_oid,
                                    repo: *mut git_repository,
                                    reference: *const c_char) -> c_int;
    
        pub fn git_commit_lookup(out: *mut *mut git_commit,
                                    repo: *mut git_repository,
                                    id: *const git_oid) -> c_int;
    
        pub fn git_commit_author(commit: *const git_commit) -> *const git_signature;
        pub fn git_commit_message(commit: *const git_commit) -> *const c_char;
        pub fn git_commit_free(commit: *mut git_commit);
    }
    
    #[repr(C)] pub struct git_repository { _private: [u8; 0] }
    #[repr(C)] pub struct git_commit { _private: [u8; 0] }
    
    #[repr(C)]
    pub struct git_error {
        pub message: *const c_char,
        pub klass: c_int
    }
    
    pub const GIT_OID_RAWSZ: usize = 20;
    
    #[repr(C)]
    pub struct git_oid {
        pub id: [c_uchar; GIT_OID_RAWSZ]
    }
    
    pub type git_time_t = i64;
    
    #[repr(C)]
    pub struct git_time {
        pub time: git_time_t,
        pub offset: c_int
    }
    
    #[repr(C)]
    pub struct git_signature {
        pub name: *const c_char,
        pub email: *const c_char,
        pub when: git_time
    }

    
Writing large **extern** blocks by hand can be a chore. If you are creating a Rust interface to a complex C library, you may want to try using the **bindgen** crate, which has functions you can use from your build script to parse C header files and generate the corresponding Rust declarations automatically. We don't have space to show **bindgen** in action here, but [bindgen's page on crates.io](https://crates.io/crates/bindgen) includes links to its documentation.
Next we'll rewrite *main.rs* completely. First, we need to declare the **raw** module: 

    mod raw;

According to **libgit2'**s conventions, fallible functions return an integer code that is positive or zero on success, and negative on failure. If an error occurs, the **git_error_last** function will return a pointer to a **git_error** structure providing more details about what went wrong. **libgit2** owns this structure, so we don't need to free it ourselves, but it could be overwritten by the next library call we make. A proper Rust interface would use **Result**, but in
the raw version, we want to use the **libgit2** functions just as they are, so we'll have to roll our own function for handling errors:

    
    #![warn(rust_2018_idioms)]
    #![allow(elided_lifetimes_in_paths)]
    
    mod raw;
    
    use std::ffi::CStr;
    use std::os::raw::c_int;
    
    
    fn check(activity: &'static str, status: c_int) -> c_int {
        if status < 0 {
            unsafe {
                // libgit2(v1.7) giterr_last(book) -> git_error_last
                let error = &*raw::git_error_last();
                println!("error while {}: {} ({})",
                        activity,
                        CStr::from_ptr(error.message).to_string_lossy(),
                        error.klass);
                std::process::exit(1);
            }
        }
    
        status
    }
    
    unsafe fn show_commit(commit: *const raw::git_commit) {
        let author = raw::git_commit_author(commit);
    
        let name = CStr::from_ptr((*author).name).to_string_lossy();
        let email = CStr::from_ptr((*author).email).to_string_lossy();
        println!("{} <{}>\n", name, email);
    
        let message = raw::git_commit_message(commit);
        println!("{}", CStr::from_ptr(message).to_string_lossy());
    }
    
    use std::ffi::CString;
    use std::mem;
    use std::ptr;
    use std::os::raw::c_char;
    
    fn main() {
        let path = std::env::args().skip(1).next()
            .expect("usage: git-toy PATH");
        let path = CString::new(path)
            .expect("path contains null characters");
    
        unsafe {
            check("initializing library", raw::git_libgit2_init());
    
            let mut repo = ptr::null_mut();
            check("opening repository", 
                raw::git_repository_open(&mut repo, path.as_ptr()));
    
            let c_name = b"HEAD\0".as_ptr() as *const c_char;
            let oid = {
                let mut oid = mem::MaybeUninit::uninit();
                check("looking up HEAD",
                    raw::git_reference_name_to_id(oid.as_mut_ptr(), repo, c_name));
                oid.assume_init()
            };
    
            let mut commit = ptr::null_mut();
            check("looking up commit",
                raw::git_commit_lookup(&mut commit, repo, &oid));
    
            show_commit(commit);
    
            raw::git_commit_free(commit);
    
            raw::git_repository_free(repo);
    
            check("shutting down library", raw::git_libgit2_shutdown());
        }
    
    }

The call to **git_repository_open** tries to open the Git repository at the given path. If it succeeds, it allocates a new **git_repository** object for it and sets **repo** to point to that. Rust implicitly coerces references into raw pointers, so passing **\&mut repo** here provides the **\*mut \*mut git_repository** the call expects.

This shows another **libgit2** convention in use (from the **libgit2** documentation):

    > Objects which are returned via the first argument as a pointer-to-pointer are owned by the caller and it is responsible for freeing them.

In Rust terms, functions like **git_repository_open** pass ownership of the new value to the caller.

It is possible to ask Rust to give us uninitialized memory, but because reading uninitialized memory at any time is instant undefined behavior, Rust provides an abstraction, **MaybeUninit**, to ease its use. **MaybeUninit\<T\>** tells the compiler to set aside enough memory for your type T, but not to touch it until you say that it's safe to do so. While this memory is owned by the **MaybeUninit**, the compiler will also avoid certain optimizations that could otherwise cause
undefined behavior even without any explicit access to the uninitialized memory in your code.
**MaybeUninit** provides a method, **as\_mut\_ptr\(\)**, that produces a **\*mut T** pointing to the potentially uninitialized memory it wraps. By passing that pointer to a foreign function that initializes the memory and then calling the unsafe method **assume\_init** on the **MaybeUninit** to produce a fully initialized **T**, you can avoid undefined behavior without the additional overhead that comes from initializing and immediately throwing away a value.
**assume\_init** is unafe because calling it on a **MaybeUninit** without being certain that the memory is actually initialized will immediately cause undefined behavior.


## A Safe Interface to libgit2

Here, then, are **libgit2**'s rules for the features the program uses:

- You must call **git_libgit2_init** before using any other library function. You must not use any library function after calling **git_libgit2_shutdown**.
- All values passed to **libgit2** functions must be fully initialized, except for output parameters.
- When a call fails, output parameters passed to hold the results of the call are left uninitialized, and you must not use their values.
- A **git_commit** object refers to the **git_repository** object it is derived from, so the former must not outlive the latter.
- Similarly, a **git_signature** is always borrowed from a given **git_commit**, and the former must not outlive the latter.
- The message associated with a commit and the name and email address of the author are all borrowed from the commit and must not be used after the commit is freed.
- Once a **libgit2** object has been freed, it must never be used again.

As it turns out, you can build a Rust interface to **libgit2** that enforces all of these rules, either through Rust's type system or by managing details internally.

Rust closures cannot serve as C function pointers: a closure is a value of some anonymous type carrying the values of whatever variables it captures or references to them; a C function pointer is just a pointers. However, Rust **fn** types work fine, as long as you declare them **extern** so that Rust knows to use the C calling conventions. The local function **shutdown** fits the bill and ensures **libgit2** gets shut down properly.
In "Unwinding" on page 158, we mentioned that it is undefined behavior for a panic to cross language boundaries. The call from **atexit** to **shutdown** can't simply use **\.expect** to handle errors reported from **raw::git_libgit2_shutdown**. Instead, it must report the error and terminate the process itself. **POSIX** forbids calling **exit** within an **atexit** handler, so **shutdown** calls **std::process::abort** to terminate the program abruptly.


## Conclusion

Rust is not a simple language. Its goal is to span two very different worlds. It's a modern programming language, safe by design, with conveniences like closures and iterators, yet it aims to put you in control of the raw capabilities of the machine it runs on, with minimal run-time overhead.


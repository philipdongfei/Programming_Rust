# Unsafe Code

*Unsafe code* lets you tell Rust, "I am opting to use features whose safety you cannot guarantee." By marking off a block or function as unsafe, you acquire the ability to call **unsafe** functions in the standard library, dereference unsafe pointers, and call functions written in other languages like C and C++, among otther powers. Rust's other safety checks still apply: type checks, lifetime checks, and bounds checks on indices all occur normally. Unsafe code just
enables a small set of additional features.
This chapter covers the essentials of working with unsafe features:

* Rust's unsafe blocks establish the boundary between ordinary, safe Rust code and code that uses unsafe features.
* You can mark functions as **unsafe**, alerting callers to the presence of extra contracts they must follow to avoid undefined behavior.
* Raw pointers and their methods allow unconstrained access to memory, and let you build data structures Rust's type system would otherwise forbid. Whereas Rust's references are safe but constrained, raw pointers, as any C or C++ programmer knows, are a powerful, sharp tool.
* Understanding the definition of undefined behavior will help you appreciate why it can have consequences far more serious than just getting incorrect results.
* Unsafe traits, analogous to **unsafe** functions, impose a contract that each implementation (rather than each caller) must follow.


## Unsafe from What?

An unsafe feature is one that imposes a *contract*: rules that Rust cannot enforce automatically, but which you must nonetheless follow to avoid *undefined behavior*.
A contract goes beyond the usual type checks and lifetime checks, imposing further rules specific to that unsafe feature. Typically, Rust itself doesn't know about the contract at all; it's just explained in the feature's documentation.
its safety checks do not detect this violation. When you use unsafe features, you, as the programmer, bear the responsibility for checking that your code adheres to their contracts.
Lots of features have rules you should follow to use them correctly, but such rules are not contracts in the sense we mean here unless the possible consequences include undefined behavior. Undefined behavior is behavior Rust firmly assumes your code could never exhibit.
You may only use unsafe features within an **unsafe** block or an **unsafe** function; we'll explain both in the sections that follow. This makes it harder to use unsafe features unknowingly: by forcing you to write an **unsafe** block or function, Rust makes sure you have acknowledged that your code may have additional rules to follow.


## Unsafe Blocks

An **unsafe** block looks just like an ordinary Rust block preceded by the **unsafe** keyword, with the difference that you can use unsafe features in the block.
Like an ordinary Rust block, the value of an **unsafe** block is that of its final expression, or () if doesn't have one.
An **unsafe** block unlocks five additional options for you:

* You can call **unsafe** functions. Each **unsafe** function must specify its own contract, depending on its purpose.
* You can dereference raw pointers. Safe code can pass raw pointers around, compare them, and create them by conversion from references (or even from integers), but only unsafe can actually use them to access memory.
* You can access the fields of **unions**, which the compiler can't be sure contain valid bit patterns for their respective types.
* You can access mutable **static** variables.
* You can access functions and variables declared through Rust's foreign function interface. These are considered **unsafe** even when immutable, since they are visible to code written in other languages that may not respect Rust's safety rules.

Restricting unsafe features to **unsafe** blocks doesn't really prevent you from doing whatever you want. It's perfectly possible to just stick an **unsafe** block into your code and move on. The benefit of the rule lies mainly in drawing human attention to code whose safety Rust can't guarantee:

* You won't accidentally use unsafe features and then discover you were responsible for contracts you didn't even know existed.
* An **unsafe** block attracts more attention from reviewers. 
* When you're considering writing an **unsafe** block, you can take a moment to ask yourself whether your task really requires such measures.

## Example: An Efficient ASCII String Type

Here's the definition of **Ascii**, a string type that ensures its contents are always valid ASCII. This type uses an unsafe feature to provide zero-cost conversion into **String**:

An **Ascii** is nothing more than a wrapper around a **Vec<u8>**, hidden inside a module that enforces extra rules about its contents. A type of this sort is called a *newtype*, a common pattern in Rust. 
At the machine level, with Rust's types out of the picture, a newtype and its element have identical representations in memory, so constructing a newtype doesn't require any machine instructions at all.


## Unsafe Functions

An unsafe function definition looks like an ordinary function definition preceded by the **unsafe** keyword. The body of an **unsafe** function is automatically considered an **unsafe** block.
You may call **unsafe** functions only within **unsafe** blocks. This means that marking a function **unsafe** warns its callers that the function has a contract they must satisfy to avoid undefined behavior.

This illustrates two critical facts about bugs and unsafe code:
- *Bugs that occur before the unsafe block can break contracts.*
- *The consequences of breaking a contract may appear after you leave the unsafe block.*

Essentially, Rust's type checker, borrow checker, and other static checks are inspecting your program and trying to construct proof that it cannot exhibit undefined behavior. When Rust compiles your program successfully, that means it succeeded in proving your code sound. An **unsafe** block is a gap in this proof: "This code," you are saying to Rust, "is fine, trust me." Whether your claim is true could depend on any part of the program that influences what happens in the **unsafe**
block, and the consequences of being wrong could appear anywhere influenced by the **unsafe** block. Writing the unsafe keyword amounts to a reminder that you are not getting the full benefit of the language's safety checks.
Given the choice, you should naturally prefer to create safe interfaces, without contracts. These are much easier to work with, since users can count on Rust's safety checks to ensure their code is free of undefined behavior. Even if your implementation uses unsafe features, it's best to use Rust's types, lifetimes, and module system to meet their contracts while using only what you can guarantee yourself, rather than passing responsibilities on to your callers.
Unfortunately, it's not unusual to come across unsafe functions in the wild whose documentation does not bother to explain their contracts.

## Unsafe Block or Unsafe Function?

You may find yourself wondering whether to use an **unsafe** block or just mark the whole function unsafe. The approach we recommend is to first make a decision about the function:

- If it's possible to misuse the function in a way that compiles fine but still causes undefined behavior, you must mark it as unsafe. The rules for using the function correctly are its contract; the existence of a contract is what makes the function unsafe.
- Otherwise, the function is safe: no well-typed call to it can cause undefined behavior. It should not be marked **unsafe**.

Whether the functioin uses unsafe features in its body is irrelevant; what matters is the presence of a contract.
Don't mark a safe function **unsafe** just because you use unsafe features in its body. Instead, use an **unsafe** block, even if it's the function's entire body.

## Undefined Behavior

We usually say that two programs are equivalent if they will always have the same visible behavior when executed: they make the same system calls, interact with foreign libraries in equivalent ways, and so on. It's a bit like a Turing test for programs: if you can't tell wheter you're interacting with the original or the translation, then they're equivalent.

It's basically impossible for Rust (or any other language) to assess whether a transformation to a program preserves its meaning unless it can trust the fundamental features of the language to behave as designed. And whether they do or not can depend not just on the code at hand, but on other, potentially distant, parts of the program. In order to do anything at all with your code, Rust assume that the rest of your program is well-behaved.
Here,then, are Rust's rules for well-behaved programs:

- The program must not read uninitialized memory.
- The program must not create invalid primitive values:
    - References,boxes, or **fn** pointers that are **null**
    - **bool** values that are not either a 0 or 1
    - **enum** values with invalid discriminant values
    - **char** values that are not valid, nonsurrogate Unicode code points
    - **str** values that are not well-formed UTF-8
    - **Fat** pointers with invalid vtables/slice lengths
    - Any value of the type **!**
- The rules for references explained in Chapter 5 must be followed. No reference may outlive its referent; shared access is read-only access; and mutable access is exclusive access.
- The program must not dereference null, incorrectly aligned, or dangling pointers.
- The program must not use a pointer to access memory outside the allocation with which the pointer is associated.
- The program must be free of data races. A data race occurs when two threads access the same memory location without synchronization, and at least one of the accesses is a write.
- The program must not unwind across a call made from another language, via the foreign function interface, as explained in "Unwinding" on page 158.
- The program must comply with the contracts of standard library functions.

Any violation of these rules constitutes undefined behavior and renders Rust's efforts to optimize your program and translate it into machine language untrustworthy.
Rust code that does not use unsafe features is guaranteed to follow all of the preceding rules, once it compiles (assuming the compiler has no bugs; we're getting there, but the curve will never intersect the asymptote). Only when you use unsafe features do these rules become your responsibility.

## Unsafe Traits

An *unsafe trait* is a trait that has a contract Rust cannot check or enforce that implementers must satisfy to avoid undefined behavior. To implement an unsafe trait, you must mark the implementation as unsafe. It is up to you to understand the trait's contract and make sure your type satisfies it.
A function that bounds its type variables with an unsafe trait is typically one that uses unsafe features itself, and satisfies their contracts only by depending on the unsafe trait's contract. An incorrect implementation of the trait could cause such a function to exhibit undefined behavior.
Note that unsafe code must not depend on ordinary, safe traits being implemented correctly.

## Raw Pointers

A *raw pointer* in Rust is an unconstrained pointer. But because raw pointers are so flexible, Rust cannot tell whether you are using them safely or not, so you can dereference them only in an **unsafe** block.
Raw pointers are Essentially equivalent to C or C++ pointers, so they're also useful for interacting with code written in those languages.
There are two kinds of raw pointers:

- A \*mut **T** is a raw pointer to a **T** that permits modifying its referent.
- A \*const **T** is a raw pointer to a **T** that only permits reading its referent.\(There is no plain \*T type; you must always specify either const or mut.\)

Although Rust implicitly dereferences safe pointer types in various situations, raw pointer dereferences must be explicit:


- The . operator will not implicitly dereference a raw pointer; you must write (*raw).field or (*raw).method(...).
- Raw pointers do not implement **Deref**, so deref coercions do not apply to them.
- Operators like == and &lt compare raw pointers as addresses: two raw pointers are equal if they point to the same location in memory. Similarly, hashing a raw pointer hashes the address it points to, not the value of its referent.
- Formatting traits like **std::fmt::Display** follow references automatically, but don't handle raw pointers at all. The exceptions are **std::fmt::Debug** and **std::fmt::Pointer**, which show raw pointers as hexadecimal addresses, without dereferencing them.

Unlike the + operator in C and C++, Rust's + does not handle raw pointers, but you can perform pointer arithmetic via their **offset** and **wrapping_offset** methods, or the more convenient **add**, **sub**, **wrapping_add**, and **wrapping_sub** methods.
Note that **as** will not convert raw pointers to references. Such conversions would be unsafe, and **as** should remain a safe operation. Instead, you must dereference the raw pointer (in an **unsafe** block) and then borrow the resulting value.
Be very careful when you do this: a reference produced this way has an unconstrained lifetime: there's no limit on how long it can live, since the raw pointer gives Rust nothing to base such a decision on.



### Dereferencing Raw Pointers Safely

Here are some common-sense guidelines for using raw pointers safely:

- Dereferencing null pointers or dangling pointers is undefined behavior, as is referring to uninitialized memory or values that have gone out of scope.
- Dereferencing pointers that are not properly aligned for their referent type is undefined behavior.
- You may borrow values out of a dereferenced raw pointer only if doing so obeys the rules for reference safety explained in Chapter 5: no reference may outlive its referent, shared access is read-only access, and mutable access is exclusive access.\(This rule is easy to violate by accident, since raw pointers are often used to create data structures with nonstandard sharing or ownership.\)
- You may use a raw pointer's referent only if it is a well-formed value of its type.
- You may use the **offset** and **wrapping_offset** methods on raw pointers only to point to bytes within the variable or heap-allocated block of memory that the original pointer referred to, or to the first byte beyond such a region.
- If you assign to a raw pointer's referent, you must not violate the invariants of any type of which the referent is a part.




### Example: RefWithFlag


    mod ref_with_flag {
        use std::marker::PhantomData;
        use std::mem::align_of;
    
        /// A `&T` and a `bool`, wrapped up in a single word.
        /// The type `T` must require at least two-byte alignment.
        ///
        /// If you're the kind of programmer who's never met a pointer whose
        /// 2^n bit you didn't want to steal, well, now you can do it safely!
        /// ("But it's not nearly as exciting this way...")
        pub struct RefWithFlag<'a, T> {
            ptr_and_bit: usize,
            behaves_like: PhantomData<&'a T> // occupies no space
        }
    
        impl<'a, T: 'a> RefWithFlag<'a, T> {
            pub fn new(ptr: &'a T, flag: bool) -> RefWithFlag<T> {
                assert!(align_of::<T>() % 2 == 0);
                RefWithFlag {
                    ptr_and_bit: ptr as *const T as usize | flag as usize,
                    behaves_like: PhantomData
                }
            }
    
            pub fn get_ref(&self) -> &'a T {
                unsafe {
                    let ptr = (self.ptr_and_bit & !1) as *const T;
                    &*ptr
                }
            }
    
            pub fn get_flag(&self) -> bool {
                self.ptr_and_bit & 1 != 0
            }
        }
    }

This code takes advantage of the fact that many types must be placed at even addresses in memory: since an even address's least significant bit is always zero, we can store something else there and then reliably reconstruct the original address just by masking off the bottom bit.

The constructor **RefWithFlag::new** takes a reference and a **bool** value, asserts that the reference's type is suitable, and then converts the reference to a raw pointer and then a **usize**. The **usize** type is defined to be large enough to hold a pointer on whatever processor we're compiliing for, so converting a raw pointer to a **usize** and back is well-defined. Once we have a **usize**, we know it must be even, so we can use the | bitwise-or operator to combine it with
the **bool**, which we've converted to an integer 0 or 1.

### Nullable Pointers

A null raw pointer in Rust is a zero address, just as in C and C++. For any type T, the **std::ptr::null<T>** function returns a **\*const T** null pointer, and **std::ptr::null_mut<T>** return a **\*mut T** null pointer.


### Type Sizes and Alignments

A value of any **Sized** type occupies a constant number of bytes in memory and must be placed at an address that is a multiple of some *alignment* value, determined by the machine architecture.
Any type's alignment is always a power of two.
A type's size is always rounded up to a multiple of its alignment, even if it technically could fit in less space.
For unsized types, the size and alignment depend on the value at hand.

### Pointer Arithmetic

Rust lays out the elements of an array, slice, or vector as a single contiguous block of memory. Elements are regularly spaced, so that if each element occupies **size** bytes, then the ith element starts with the **i \* size**th byte.
One nice consequence of this is that if you have two raw pointers to elements of an array, comparing the pointers gives the same results as comparing the elements' indices: if i \< j, then a raw pointer to the ith element is less than a raw pointer to the jth element. This makes raw pointers useful as bounds on array traversals. In fact, the standard library's simple iterator over a slice was originally defined like this:

    struct Iter<'a, T> {
        ptr: *const T,
        end: *const T,
        ...
    }

The **ptr** field points to the next element iteration should produce, and the **end** field serves as the limit: when **ptr == end**, the iteration is complete.

### Moving into and out of Memory

### Example: GapBuffer

### Panic Safety in Unsafe Code

## Reinterpreting Memory with Unions

## Borrowing Unions



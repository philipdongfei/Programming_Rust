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

## Unsafe Block or Unsafe Function?

## Undefined Behavior

## Unsafe Traits

## Raw Pointers

### Dereferencing Raw Pointers Safely

### Example: RefWithFlag

### Nullable Pointers

### Type Sizes and Alignments

### Pointer Arithmetic

### Moving into and out of Memory

### Example: GapBuffer

### Panic Safety in Unsafe Code

## Reinterpreting Memory with Unions

## Borrowing Unions



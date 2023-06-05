# Iterators

An *iterator* is a value that produces a sequence of values, typically for a loop to operate on. 

> trait.Iterator.fold() takes two arguments: an initial value, and a closure with two arguments: an 'accumulator', and an element. The closure returns the value that the accumulator should have for the next iteration. The initial value is the value the accumulator will have on the first call.


In this chapter, we'll explain:

- The Iterator and IntoIterator traits, which are the foundation of Rust's iterators
- The three stages of a typical iterator pipeline: creating an iterator from some sort of value source; adapting one sort of iterator into another by selecting or processing values as they go by; and then consuming the values the iterator produces
- How to implement iterators for your own types


## The Iterator and IntoIterator Traits

An iterator is any value that implements the *std::iter::Iterator* trait:

    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
        ... // many default methods
    }

**Item** is the type of value the iterator produces. The **next** method either returns **Some(v)**, where **v** is the iterator's next value, or returns **None** to indicate the end of the sequence. 

If there's a natural way to iterate over some type, that type can implement *std::iter::IntoIterator**, whose **into_iter** method takes a value and returns an iterator over it:

    trait IntoIterator where Self::IntoIter: Iterator<Item=Self::Item> {
        type Item;
        type IntoIter: Iterator;
        fn into_iter(self) -> Self::IntoIter;
    }

**IntoIter** is the type of the iterator value itself, and **Item** is the type of value it produces. We call any type that implements **IntoIterator** an *iterable*, because it's something you could iterate over if you asked.

With this example in mind, here's some terminology for iterators:

- As we've said, an *iterator* is any type that implements **Iterator**.
- An *iterable* is any type that implements **IntoIterator**: you can get an iterator over it by calling its **into_iter** method. The vector reference &v is the iterable in this case.
- An iterator *produces* values.
- The values an iterator produces are *items*.
- The code that receives the items an iterator produces is the *consumer*. 


## Creating Iterators


### iter and iter_mut Methods

If there's more than one common way to iterator over a type, the type usually provides specific methods for each sort of traversal, since a plain **iter** method would be ambiguous.


### IntoIterator Implementations

Most collections actually provide several implementations of **IntoIterator**, for shared references **(&T)**, mutable references **(&mut T)**, and moves **(T)**:

- Given a *shared reference* to the collection, **into_iter** returns an iterator that produces shared references to its items.
- Given a *mutable reference* to the collection, **into_iter** returns an iterator that produces mutable references to the items.
- When passed the collection *by value*, **into_iter** returns an iterator that takes ownership of the collection and returns items by value; the item's ownership moves from the collection to the consumer, and the original collection is consumed in the process.

Since a **for** loop applies **IntoIterator::into_iter** to its operand, these three implementations are what create the following idioms for iterating over shared or mutable references to a collection, or consuming the collection and taking ownership of its elements:

    for element in &collection {...}
    for element in &mut collection {...}
    for element in collection {...}

Note every type provides all three implementations.

Slices implement two of the three **IntoIterator** variants; since they don't own their elements, there is no "by value" case.


**IntoIterator** can also be useful in generic code: you can use a bound like **T**: **IntoIterator** to restrict the type variable **T** to types that can be iterated over. Or, you can write **T**: **IntoIterator<Item=U>** to further require the iteration to produce a particular type **U**.



### from_fn and successors

Given a function returning **Option<T>**, **std::iter::from_fn** returns an iterator that simply calls the function to produce its items.

A note of caution: the **from_fn** and **successors** methods are flexible enough that you could turn pretty much any use of iterators into a single call to one or the other, passing complex closures to get the behavior you need. But doing so neglects the opportunity that iterators provide to clarify how data flows through the computation and use standard names for common patterns. Make sure you've familiarized yourself with the other iterator methods
in this chapter before you lean on these two; there are often nicer ways to get the job done.


### drain Methods

Many collection types provide a **drain** method that takes a mutable reference to the collection and returns an iterator that passes ownership of each element to the consumer.

### Other Iterator Sources


## Iterator Adapters

Once you have an iterator in hand, the **Iterator** trait provides a broad selection of *adapter methods*, or simply *adapters*, that consume one iterator and build a new one with useful behaviors.

### map and filter

A chain of iterator adapters is like a pipeline in the Unix shell: each adapter has asingle purpose, and it's clear how the sequence is being transformed as one reads from left to right.

These adapters' signatures are as follows:
    
    fn map<B, F>(self, f: F) -> impl Iterator<Item=B>
        where Self: Sized, F: FnMut(Self::Item) -> B;

    fn filter<P>(self, predicate: P) -> impl Iterator<Item=Self::Item>
        where Self: Sized, P: FnMut(&Self::Item) -> bool;

the method returns an **Iterator** that produces items of the given type.

Since most dapaters take **self** by value, they require **Self** to be **Sized** (which all common iterators are).

A **map** iterator passes each item to its closure by value and, in turn, passes along ownership of the closure's result to its consumer. A **filter** iterator passes each item to its closure by shared reference, retaining ownership in case the item is selected to be passed on to its consumer. This is why the xample must dereference s to compare it with "ignuanas": the **filter** iterator's item type is **&str**, so the type of the closure's argument s is
**&&str**.

There are two important points to notice about iterator adapters.

First, simple calling an adapter on an iterator doesn't consume any items; it just returns a new iterator, ready to produce its own items by drawing from the first iterator as needed. In a chain of adapters, the only way to make any work actually get done is to call **next** on the final iterator.

The second important point is that iterator adapters are a zero-overhead abstraction.


### filter_map and flat_map

### flatten

### take and take_while

### skip and skip_while

### peekable

### fuse

### Reversible Iterators and rev

### inspect

### chain

### enumerate

### zip

### by_ref

### cloned, copied

### cycle

## Consuming Iterators

## Implementing Your Own Iterators


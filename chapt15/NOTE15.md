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

### from_fn and successors

### drain Methods

### Other Iterator Sources


## Iterator Adapters

## Consuming Iterators

## Implementing Your Own Iterators


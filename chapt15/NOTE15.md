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

The **filter_map** adapter is similar to **map** except that it lets its closure either transform the item into a new item (as **map** does) or drop the item from the iteration.

    fn filter_map<B, F>(self, f: F) -> impl Iterator<Item=B>
        where Self: Sized, F: FnMut(Self::Item) -> Option<B>;

You can think of the **flat_map** adapter as continuing in the same vein as **map** and **filter_map**, except that now the closure can return ont just one item (as with map) or zero or one items (as with **filter_map**), but a sequence of any number of items.
The signature of **flat_map** is shown here:
    
    fn flat_map<U, F>(self, f: F) -> impl Iterator<Item=U::Item>
        where F: FnMut(Self::Item) -> U, U: IntoIterator;

The closure passed to **flat_map** must return an iterable, but any sort of iterable will do[1].

[1]: In fact, since **Option** is an iterable behaving like a sequence of zero or one items, **iterator.filter_map(closure)** is equivalent to **iterator.flat_map(closure)**, assuming **closure** returns an **Option<T>**.


But remember that iterators are lazy: it's only the **for** loop's calls to the **flat_map** iterator's **next** method that cause work to be done. The full concatenated sequence is never constructed in memory. Instead, what we have here is a little state machine that draws from the city iterator, one item at a time, until it's exhausted, and only then produces a new city iterator for the next country. The effect is that of a nested loop, but packaged up for use as an iterator.


### flatten

The **flatten** adapter concatenates an iterator's items, assuming each item is itself an iterable.

The name "flatten" comes from the image of flattening a two-level structure into a one-level structure: the **BTreeMap** and its **Vecs** of names are flattened into an iterator producing all the names.

The signature of **flatten** is as follows:
    
    fn flatten(self) -> impl Iterator<Item=Self::Item::Item>
        where Self::Item: IntoIterator;

In other words, the underlying iterator's items must themselves implement **IntoIterator** so that it is effectively a sequence of sequences.The **flatten** method then returns an iterator over the concatenation of those sequences.

The **flatten** method gets used in a new surprising ways. If you have a **Vec<Option<...>>** and you want to iterate over only the **Some** values, **flatten** works beautifully:

    assert_eq!(vec![None, Some("day"), None, Some("one")]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>(),
        vec!["day", "one"]);

This works because **Option** itself implements **IntoIterator**, representing a sequence of either zero or one elements. The **None** elements contribute nothing to the iteration, whereas each **Some** element contributes a single value. Similarly, you can use **flatten** to iterator over **Option<Vec<...>>** values: **None** behaves the same as an empty vector.

**Result** also implements **IntoIterator**, with **Err** representing an empty sequence, so applying **flatten** to an iterator of **Result** values effectively squeezes out all the **Err**s and throws them away, resulting in a stream of the unwrapped success values.


### take and take_while

The **Iterator** trait's **take** and **take_while** adapters let you end an iteration after a certain number of items or when a closure decides to cut things off. Their signatures are as follows:

    fn take(self, n: usize) -> impl Iterator<Item=Self::Item>
        where Self: Sized;

    fn take_while<P>(Self, predicate: P) -> impl Iterator<Item=Self::Item>
        where Self: Sized, P: FnMut(&Self::Item) -> bool;

Both take ownership of an iterator and return a new iterator that passes along items from the first one, possibly ending the sequence earlier. The **take** iterator returns **None** after producing at most n items. The **take_while** iterator applies **predicate** to each item and returns **None** in place of the first item for which **predicate** returns **false** and on every subsequent call to **next**.


### skip and skip_while

The **Iterator** trait's **skip** and **skip_while** methods are the complement of **take** and **take_while**: they drop a certain number of items from the beginning of an iteration, or drop items until a closure finds one acceptable, and then pass the remaining items through unchanged. Their signatures are as follows:

    fn skip(self, n: usize) -> impl Iterator<Item=Self::Item>
        where Self: Sized;
    fn skip_while<P>(self, predicate: P) -> impl Iterator<Item=Self::Item>
        where Self: Sized, P: FnMut(&Self::Item) -> bool;


### peekable

A peekable iterator lets you peek at the next item that will be produced without actually consuming it. You can turn any iterator into a peekable iterator by calling the **Iterator** trait's **peekable** method:

    fn peekable(self) -> std::iter::Peekable<Self>
        where Self: Sized;

Here, **Peekable<Self>** is a **struct** that implements **Iterator<Item=Self::Item>**, and **Self** is the type of the underly iterator.

A **Peekable** iterator has an additional method **peek** that returns an **Option<&Item>**: **None** if the underlying iterator is done and otherwise **Some(r)**, where r is a shared reference to the next item.(Note that if the iterator's item type is already a reference to something, this ends up being a reference to a reference.)

Calling **peek** tries to draw the next item from the underlying iterator, and if there is one, caches it until the next call to **next**. 

### fuse

The **fuse** adapter takes any iterator and produces one that will definitely continue to return **None** once it has done so the first time.

The **fuse** adapter is probably most useful in generic code that needs to work with iterators of uncertain origin.


### Reversible Iterators and rev

Some iterators are able to draw items from both ends of the sequence. You can reverse such iterators by using the **rev** adapter. 
Such iterators can implement the **std::iter::DoubleEndedIterator** trait, which extends **Iterator**:

    trait DoubleEndedIterator: Iterator {
        fn next_back(&mut self) -> Option<Self::Item>;
    }

If an iterator is double-ended, you can reverse it with the **rev** adapter:

    fn rev(self) -> impl Iterator<Item=Self>
        where Self: Sized + DoubleEndedIterator;


### inspect

The **inspect** adapter is handy for debugging pipelines of iterator adapters, but it isn't used much in production code.

### chain

The **chain** adapter appends one iterator to another. More precisely, *i1.chain(i2)* returns an iterator that draws items from *i1* until it's exhausted and then draws items from *i2*.
The **chain** adapter's signature is as follows:
    
    fn chain<U>(self, other: U) -> impl Iterator<Item=Self::Item>
        where Self: Sized, U: IntoIterator<Item=Self::Item>;


### enumerate

The **Iterator** trait's **enumerate** adapter attaches a running index to the sequence, taking an iterator that produces items A, B, C, ... and returning an iterator that produces pairs(0, A), (1, B), (2, C), 

### zip

The **zip** adapter combines two iterators into a single iterator that produces pairs holding one value from each iterator, like a zipper joining its two sides into a single seam. The zipped iterator ends when either of the two underlying iterators ends.

whereas **enumerate** attaches indices to the sequence, **zip** attaches any arbitrary iterator's items.
The argument to **zip** doesn't need to be an iterator itself; it can be any iterable.

### by_ref

An iterator's **by_ref** method borrows a mutable reference to the iterator so that you can apply adapters to the reference. When you're done consuming items from these adapters, you drop them, the borrow ends, and you regain access to your original iterator.

The **by_ref** adapter's definition is trivial: it returns a mutable reference to the iterator. Then, the standard library includes this strange little implementation:
    
    impl<'a, I: Iterator + ?Sized> Iterator for &'a mut I {
        type Item = I::Item;
        fn next(&mut self) -> Option<I::Item> {
            (**self).next()
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            (**self).size_hint()
        }
    }

In other words, if **I** is some iterator type, then **&mut I** is an iterator too, whose **next** and **size_hint** methods defer to its referent. When you call an adapter on a mutable reference to an iterator, the adapter takes ownership of the *reference*, not the iterator itself. That's just a borrow that ends when the adapter goes out of scope.


### cloned, copied

The **cloned** adapter takes an iterator that produces references and returns an iterator that produces values cloned from those references, much like **iter.map(|item| item.clone())**. Naturally, the reference type must implement **Clone**.

The **copied** adapter is the same idea, but more restrictive: the referent type must implement **Copy**. A call like **iter.copied()** is roughly the same as **iter.map(|r| *r)**.


### cycle

The **cycle** adapter returns an iterator that endlessly repeats the sequence produced by the underly iterator. The underlying iterator must implement **std::clone::Clone** so that **cycle** can save its initial state and reuse it each time the cycle starts again.


## Consuming Iterators

here we finish off the process by showing ways to consume them.

### Simple Accumulation: count, sum, product

The **count** method draws items from an iterator until it return **None** and tells you how many it got.
The **sum** and **product** methods compute the sum or product of the iterator's items, which must be integers or floating-point numbers.

### max, min

The **min** and **max** methods on **Iterator** return the least or greastest item the iterator produces. The iterator's item type must implement **std::cmp::Ord** so that items can be compared with one another.

Rust's floating-point types **f32** and **f64** implement only **std::cmp::PartialOrd**, not **std::cmp::Ord**, so you can't use the **min** and **max** methods to compute the least or greatest of a sequence of floating-point numbers.

### max_by, min_by

The **max_by** and **min_by** methods return the maximum or minimum item the iterator produces, as determined by a comparison function you provide.

### max_by_key, min_by_key

The **max_by_key** and **min_by_key** methods on **Iterator** let you select maximum or minimum item as determined by a closure applied to each item. The closure can select some field of the item or perform a computation on the items.

    fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where Self: Sized, F: FnMut(&Self::Item) -> B;
    fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
        where Self: Sized, F: FnMut(&Self::Item) -> B;


### Comparing Item Sequences

Although iterators do not support Rust's comparison operators, they do provide methods like **eq** and **lt** that do the same job, drawing pairs of items from the iterators and comparing them until a decision can be reached.

### any and all

The **any** and **all** methods apply a closure to each item the iterator produces and return **true** if the closure returns **true** for any item, or for all the items.

### position, rposition, and ExactSizeIterator

The **position** method applies a closure to each item from the iterator and returns the index of the first item for which the closure returns **true**. More precisely, it returns an **Option** of the index: if the closure returns **true** for no item, **position** returns **None**. It stops drawing items as soon as the closure returns **true**.

The rposition method is the same, except that it searches from the right.

An exact-size iterator is one that implements the **std::iter::ExactSizeIterator** trait:

    trait ExactSizeIterator: Iterator {
        fn len(&self) -> usize { ... }
        fn is_empty(&self) -> bool { ... }
    }

### fold and rfold

The **fold** method is a very general tool for accumulating some sort of result over the entire sequence of items an iterator produces. Given an initial value, which we'll call the *accumulator*, and a closure, **fold** repeatedly applies the closure to the current accumulator and the next item from the iterator. The value the closure returns is taken as the new accumulator, to be passed to the closure with the next item. The final accumulator value is what **fold** itself returns. If thesequence is empty, **fold** simply returns the initial accumulator.

The **fold** method's signature is as follows:

    fn fold<A, F>(self, init: A, f: F) -> A
        where Self: Sized, F: FnMut(A, Self::Item) -> A;

Here, **A** is the accumulator type. The **init** argument is an **A**, as is the closure's first argument and return value, and the return value of **fold** itself.

### try_fold and try_rfold

The **try_fold** method is the same as **fold**, except that the process of iteration can exit early, without consuming all the values from the iterator. The closure you pass to **try_fold** must return a **Result**: if it returns **Err(e)**, **try_fold** returns immediately with **Err(e)** as its value. Otherwise, it continues folding with the success value. The closure can also return an **Option**: returning **None** exits early, and the result is an **Option** of the folded value.

### nth, nth_back

The **nth** method takes an index n, skips that many items from the iterator, and returns the next item, or **None** if the sequence ends before that point. Calling **.nth(0)** is equivalent to **.next()**.

Its signature is shown here:

    fn nth(&mut self, n: usize) -> Option<Self::Item>
        where Self: Sized;

The **nth_back** method is much the same, except that it draws from the back of a doble-ended iterator.

### last

The **last** method returns the last item the iterator produces, or **None** if it's empty. Its signature is as follows:

    fn last(self) -> Option<Self::Item>;


### find, rfind, and find_map

The **find** method draws items from an iterator, returning the first item for which the given closure return **true**, or **None** if the sequence ends before a suitable item is found. Its signature is:

    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where Self: Sized,
            P: FnMut(&Self::Item) -> bool;

The **rfind** method is similar, but it requires a double-ended iterator and searches values from back to front, returning the *last* item for which the closure returns **true**.

Sometimes your closure isn't just a simple predicate casting a Boolean judgment on each item and moving on: it might be something more complex that produces an interesting value in its own right. In this case, **find_map** is just what you want. Its signature is:

    fn find_map<B, F>(&mut self, f: F) -> Option<B> where
        F: FnMut(Self::Item) -> Option<B>;

### Building Collections: collect and FromIterator

Naturally, **collect** itself doesn't know how to construct all these types. Rather, when some collection type like **Vec** or **HashMap** knows how to construct itself from an iterator, it implements the **std::iter::FromIterator** trait, for which **collect** is just a convenient veneer:

    trait FromIterator<A>: Sized {
        fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self;
    }

If a collection type implements **FromIterator<A>**, then its type-associated function **from_iter** builds a value of that type from an iterable producing items of type A.

This is where the **Iterator** trait's **size_hint** method comes in:

    trait Iterator {
        ...
        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, None)
        }
    }

This method returns a lower bound and optional upper bound on the number of items the iterator will produce. The default definition returns zero as the lower bound and declines to name an upper bound, saying, in effect, "I have no idea," but many iterators can do better than this.


### The Extend Trait

If a type implements the **std::iter::Extend** trait, then its **extend** method adds an iterable's items to the collection. 
All of the standard collections implement **Extend**, so they all have this method; so does **String**. Arrays and slices, which have a fixed length, do not.

The trait's definition is as follows:

    trait Extend<A> {
        fn extend<T>(&mut self, iter: T)
            where T: IntoItterator<Item=A>;
    }


### partition

The **partition** method divides an iterator's items among two collections, using a colsure to decide where each item belongs.

Like **collect**, **partition** can make any sort of collections you like, although both must be of the same type. And like **collect**, you'll need to specify the return type.

The signature of **partition** is as follows:

    fn partition<B, F>(self, f: F) -> (B, B)
        where Self: Sized,
              B: Default + Extend<Self::Item>,
              F: FnMut(&Self::Item) -> bool;


### for_each and try_for_each

The **for_each** method simply applies a closure to each item.

If your closure needs to be fallible or exit early, you can use **try_for_each**.

## Implementing Your Own Iterators

Iterators are the embodiment of Rust's philosophy of providing pwoerful, zero-cost abstractions that improve the expressiveness and readability of code. Iterators don't replace loops entirely, but they do provide a capable primitive with build-in lazy evaluation and excellent performance.


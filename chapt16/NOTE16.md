# Collections

The Rust standard library contains several *collections*, generic types for storing data in memory. Before we begin, let's address a few systematic differences between Rust's collections and those in other languages.

- First, moves and borrowing are everywhere. 
- Second, Rust doesn't have invalidation errors--the kind of dangling-pointer bug where a collection is resized, or otherwise changed, while the program is holding a pointer to data inside it.
- Finally, Rust does not have *null*, so we'll see *Options* in places where other languages would use *null*.

## Overview

This chapter discusses each collection type in turn:

* **Vec<T>**
    A growable, heap-allocated array of values of type **T**. About half of this chapter is dedicated to **Vec** and its many useful methods.

* **VecDeque<T>**
    Like **Vec<T>**, but better for use as a first-in-first-out queeu. It 
    supports efficiently adding and removing values at the front of the
    list as the back. This comes at the cost of making all other operations
    slightly slower.

* **BinaryHeap<T>**
    A priority queue. The values in a **BinaryHeap** are organized so that 
    it's always efficient to find and remove the maximum value.

* **HashMap<K, V>**
    A table of key-value pairs. Looking up a value by its key is fast. The
    entries are stored in an arbitrary order.

* **BTreeMap<K, V>**
    Like **HashMap<K, V>**, but it keeps the entries sorted by key. A 
    **BTreeMap<String, i32>** stores its entries in **String** comparison
    order. Unless you need the entries to stay sorted, a **HashMap** is
    faster.

* **HashSet<T>**
    A set of values of type **T**. Adding and removing values is fast, and
    it's fast to ask whether a given value is in the set or not.

* **BTreeSet<T>**
    Like **HashSet<T>**, but it keeps the elements sorted by value. Again,
    unless you need the data sorted, a **HashSet** is faster.


## Vec<T>

a vector has three fields: the length, the capacity, and a pointer to a heap allocation where the elements are stored. The empty vector, **numbers**, initially has a capacity of 0. No heap memory is allocated for it until the first element is added.


### Accessing Elements

Rust is picky about numeric types, and it makes no exceptions for vectors. Vector lengths and indices are of type **usize**. Trying to use a **u32**, **u64**, or **isize** as a vector index is an error.
Several methods provide easy access to particular elements of a vector or slice (note that all slice methods are available on arrays and vectors too):

* **slice.first()**
    Returns a reference to the first element of **slice**, if any.
    The return type is **Option<&T>**, so the return value is **None**
    if **slice** is empty and **Some(&slice[0])** if it's not empty.
* **slice.last()**
    Similar but returns a reference to the last element.
* **slice.get(index)**
    Returns **Some** reference to **slice[index]**, if it exits. If 
    **slice** has fewer than **index+1** elements, this returns **None**.
* **slice.first_mut()**, **slice.last_mut()**, **slice.get_mut(index)**
    Variations of the preceding that borrow *mut* references.
    Because returning a **T** by value would mean moving it, methods that
    access elements in place typically return those elements by reference.
    An exception is the **.to_vec()** method, which makes copies.
* **slice.to_vec()**
    Clones a whole slice, returning a new vector
    This method is available only if the elements are cloneable, that is,
    **where T: Clone**.


### Iteration

Vectors and slices are iterable, either by value or by reference, following the pattern described in "IntoItertor Implementations"

- Iterating over a **Vec<T>** produces items of type **T**. The elements are moved out of the vector one by one, consuming it.
- Iterating over a value of type **&[T; N]**, **&[T]**, or **&Vec<T>**--that is, a reference to an array, slice, or vector--produces items of type **&T**, references to the individual elements, which are not moved.
- Iterating over a value of type **&mut [T; N]**, **&mut [T]**, or **&mut Vec<T>** produces items of type **&mut T**.

### Growing and Shrinking Vectors

The *length* of an array, slice, or vector is the number of elements it contains.
The remaining methods in this section are about growing and shrinking vectors. They are not present on arrays and slices, which can't be resized once created.
All of a vector's elements are stored in a contiguous, heap-allocated chunk of memory. The *capacity* of a vector is the maximum number of elements that would fit in this chunk. **Vec** normally manages the capacity for you , automatically allocating a larger buffer and moving the elements into it when more space is needed.

**slice.len()**
**slice.is_empty()**
**Vec::with_capacity(n)**
**vec.capacity()**
**vec.reserve(n)**
**vec.reserve_exact(n)**
**vec.shrink_to_fit()**
These two methods add or remove a single value at the end of a vector:
**vec.push(value)**
    Adds the given value to the end of *vec*.
**vec.pop()**
    Removes and returns the last element. The return type is 
    **Option<T>**. This returns **Some(x)** if the popped element is x and
    **None** if the vector was already empty.
Note that **.push()** takes its argument by value, not by reference. Likewise, **.pop()** returns the popped value, not a reference.
**vec.insert(index, value)**
**vec.remove(index)**
**vec.resize(new_len, value)**
**vec.resize_with(new_len, closure)**
**vec.truncate(new_len)**
**vec.clear()**
**vec.extend(iterable)**
**vec.split_off(index)**
**vec.append(&mut vec2)**
**vec.drain(range)**
**vec.retain(test)**
**vec.dedup()**
    Drops repeated elements.
    Note that there are still two 's' characters in the output. This 
    method only removes *adjacent* duplicates. To eliminate all duplicates,
    you have three options: sort the vector before calling **.dedup()**, 
    move the data into a set, or (to keep the elements in their original 
    order) use this **.retain()** trick.
**vec.dedup_by(same)**
**vec.dedup_by_key(key)**


### Joining

Two methods work on *arrays of arrays*, by which we mean any array, slice, or vector whose elements are themselves arrays, slices, or vectors.


### Splitting

Rust has several methods that can borrow **mut** references to two or more parts of an array, slice, or vector at once. Unlike the preceding code, these methods are safe, because by design, they always split the data into *nonoverlapping* regions.
None of these methods directly modifies an array, slice, or vector; they merely return new references to parts of the data inside.

**slice.split(is_sep)**, **slice.split_mut(is_sep)**
    Split **slice** into one or more subslices, using the function or 
    closure **is_sep** to figure out where to split. They return an
    iterator over the subslices.
    As you consume the iterator, it calls **is_sep(&element)** for each
    element in the slice. If **is_sep(&element)** is true, the element
    is a separator. Separators are not include in any output subslice.
    The output always contains at least one subslice, plus one per
    separator. Empty subslices are included whenever separators appear
    adjacent to each other or to the ends of **slice**.
**slice.splitn(n, is_sep)**, **slice.rsplit_mut(is_sep)**
    The same but they produce at most n subslices. After the first n-1
    slices are found, **is_sep** is not called again. The last subslice
    contains all the remaining elements.
**slice.windows(n)**
    Returns an iterator that behaves like a "sliding window" over the
    data in **slice**.
    If n is greater than the length of **slice**, then no slices are
    produced. If n is 0, the method panics.
    Because the subslices overlap, there is no variation of this method 
    that returns mut references.

### Swapping

**slice_a.swap(&mut slice_b)**
    Swaps the entire contents of **slice_a** and **slice_b.slice_a** and
    **slice_b** must be the same length.

**vec.swap_remove(i)**
    Removes and returns vec[i]. This is like **vec.remove(i)** except that
    instead of sliding the rest of the vector's elements over to close the
    gap, it simply moves vec's last element into the gap. It's useful when
    you don't care about the order of the items left in the vector.

### Sorting and Searching

**slice.sort_by(cmp)**
    Sorts the elements of **slice** using a function or closure **cmp** to
    specify the sort order.**cmp** must implement 
    **Fn(&T, &T) -> std::cmp::Ordering**.

**slice.sort_by_key(key)**
    Sorts the elements of slice into increasing order by a sort key, given
    by the function or closure key. The type of key must implement
    **Fn(&T) -> K** where **K: Ord**.
    This is useful when T contains one or more ordered fields, so that it
    could be sorted multiple ways.
    Note that these sort-key values are not cached during sorting, so the
    **key** function may be called more than *n* times.
    For technical reasons, **key(element)** can't return any references
    borrowed from the element.

**slice.binary_search(&value)**, **slice.binary_search_by(&value, cmp)**,
**slice.binary_search_by_key(&value, key)**
    All search for value in the given sorted slice. Note that value is 
    passed by reference.
    The return type of these methods is **Result<usize, uszie>**. They
    return **Ok(index)** if **slice[index]** equals **value** under the
    specified sort order. If there is no such index, then they return
    **Err(insertion_point)** such that inserting **value** at 
    **insertion_point** would preserve the order.
Of course, a binary search only works if the slice is in fact sorted in the specified order. Otherwise, the results are arbitrary--garbage in, garbage out.
**slice.contains(&value)**
    Returns **true** if any element of **slice** is equal to **value**.
    This simply checks each element of slice until a match is found. 
    Again, **value** is passed by reference.


### Comparing Slices

If a type **T** supports the == and != operators, then arrays **[T; N]**, slices **[T]**, and vectors **Vec<T>** support them too. Two slices are equal if they're the same length and their corresponding elements are equal.
If **T** supports the operators **<, <=, >**, and **>=**, then arrays, slices, and vectors of **T** do too.


### Random Elements

Random numbers are not build into the Rust standard library. The **rand** crate, which provides them, offers these two methods for getting random output from an array, slice, or vector.

### Rust Rules Out Invalidation Errors

Most mainstream programming languages have collections and iterators, and they all have some variation on this rule: don't modify a collection while you're iterating over it.

Having an error pointed out to you is nice, but of course, you still need to find a way to get the desired behavior! The easiest fix here is to write:

    my_vec.retain(|&val| val <= 4);

Or, you can do what you'd do in Python or any other language: create a new vector using a **filter**.


## VecDeque<T>

Rust's **std::collections::VecDeque<T>** is a *deque*, a double-ended queue. 
The implementation of **VecDeque** is a ring buffer.
Because deques don't store their elements contiguously in memory, they can't inherit all the methods of slices.


## BinaryHeap<T>

A **BinaryHeap** is a collection whose elements are kept loosely organized so that the greatest value always bubbles up to the front of the queue.

This makes **BinaryHeap** useful as a work queue. You can define a task struct that implements **Ord** on the basis of priority so that higher-priority tasks are **Greater** than lower-priority tasks. Then, create a **BinaryHeap** to hold all pending tasks. Its **.pop()** method will always return the most important item, the task your program should work on next.

Note: **BinaryHeap** is iterable, and it has an **.iter()** method, but the iterators produce the heap's elements in an arbitrary order, not from greatest to least. To consume values from a **BinaryHeap** in order of priority, use a while loop.


## HashMap<K, V> and BTreeMap<K, V>

A *map* is a collection of key-value pairs (called *entries*). No two entries have the same key, and the entries are kept organized so that if you have a key, you can efficiently look up the coresponding value in a map. In short, a map is a lookup table.
The Rust standard library uses B-trees rather than balanced binary trees because B-trees are faster on modern hardware. A binary tree may use fewer comparisons per search than a B-tree, but searching a B-tree has better *locality*--that is, the memory accesses are grouped together rather than scattered across the whole heap. This makes CPU cache misses rarer. It's a significant speed boost.

* **HashMap::with_capacity(n)**
    Creates a new, empty hash map with room for at least *n* entries. 
    **HashMap**s, like vectors, store their data in a single heap
    allocation, so they have a capacity and the related methods 
    **hash_map.capacity()**, **hash_map.reserve(additional)**, and 
    **hash_map.shrink_to_fit()**. **BTreeMap**s do not.

* **map.get_mut(&key)**
    Similar, but it returns a mut reference to the value.
    In general, maps let you have *mut* access to the values stored inside
    them, but not the keys. The values are yours to modify however you like
    . The keys belong to the map itself; it needs to ensure that they don't
    change, because the entries are organized by their keys. Modifying a 
    key in-place would be a bug.

* **btree_map.split_off(&key)**
    Splits **btree_map** in two. Entries with keys less the key are left
    in **btree_map**. Returns a new **BTreeMap<K, V>** containing the other
    entries.

### Entries

Both **HashMap** and **BTreeMap** have a corresponding **Entry** type. The point of entries is to eliminate redundant map lookups.

The idea with entries is that we do the lookup just once, producing an **Entry** value that is then used for all subsequent operations. This one-liner is equivalent to all the preceding code, except that it does the lookup only once:

    let record = student_map.entry(name.to_string()).or_insert_with(Student::new);

The **Entry** value returned by **student_map.entry(name.to_string())** acts like a mutable reference to a place within the map that's either *occupied* by a key-value pair, or *vacant*, meaning there's no entry there yet. If vacant, the entry's **.or_insert_with()** method inserts a new **Student**. Most uses of entries are like this: short and sweet.


### Map Iteration
    
* Iterating by value **(for (k, v) in map)** produces **(K, V)** pairs. This consumes the map.
* Iterating over a shared reference **(for (k, v) in &map)** produces **(&K, &V)** pairs.
* Iterating over a mut reference **(for (k, v) in &mut map)** produces **(&K, &mut V)** pairs.

## HashSet<T> and BTreeSet<T>

*Sets* are collections of values arranged for fast membership testing.
A set never contains multiple copies of the same value.

### Set Iteration

There are two ways to iterate over sets:
* Iterating by value (**"for v in set"**) produces the members of the set (and consumes the set).
* Iterating by shared reference (**"for v in &set"**) produces shared references to the members of the set.

Iterating over a set by **mut** reference is not supported. There's no way to get a **mut** reference to a value stored in a set.

### When Equal Values Are Different

Sets have a few odd methods that you need to use only if you care about differences between "equal" values.
Such differences do often exist. Two identical **String** values, for example, store their characters in different locations in memory.


### Whole-Set Operations

So far, most of the set methods we've seen are focused on a single value in a single set. Sets also have methods that operate on whole sets.


## Hashing

**HashMap** keys and **HashSet** elements must implement both **Hash** and **Eq**.
Most built-in types that implement **Eq** also implement **Hash**. The integer types, char, and String are all hashable; so are tuples, arrays, slices, and vectors, as long as their elements are hashable.
One principle of the standard library is that a value should have the same hash code regardless of where you store it or how you point to it.
Structs and enums don't implement Hash by default, but an implementation can be derived:

    /// The ID number for an object in the British Museum's collection.
    #[derive(Clone, PartialEq, Eq, Hash)]
    enum MuseumNumber {
        ...
    }

This works as long as the type's fields are all hashable.
If you implement PartialEq by hand for a type, you should also implement Hash by hand.

## Using a Custom Hashing Algorithm

The hash method is generic, so the Hash implementations shown earlier can feed data to any type that implements Hasher. This is how Rust supports pluggable hashing algorithms.
A third trait, **std::hash::BuildHasher**, is the trait for types that reperesent the initial state of a hashing algorithm. Each Hasher is single use, like an iterator: you use it once and throw it away. A **BuildHasher** is reusable.
Every **HashMap** contains a **BuildHasher** that is uses each time it needs to compute a hash code. The **BuildHasher** value contains the key, initial state, or other parameters that the hashing algorithm needs every time it runs.
Rust's default hashing algorithm is a well-know algorithm called SipHash-1-3. SipHash is fast, and it's very good at minimizing hash collisions. In fact, it's a crypto-graphic algorithm: there's no known efficient way to generate SipHash-1-3 collisions.


## Beyond the Standard Collections

For now, we'll just bask in the warm glow of the standard collections and their safe, efficient APIs. Like much of the Rust standard library, they're designed to ensure that the need to write unsafe is as rare as possible.




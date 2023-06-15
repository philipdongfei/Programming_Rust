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

### Swapping

### Sorting and Searching

### Comparing Slices

### Random Elements

### Rust Rules Out Invalidation Errors


## VecDeque<T>

## BinaryHeap<T>

## HashMap<K, V> and BTreeMap<K, V>

## HashSet<T> and BTreeSet<T>

## Hashing

## Using a Custom Hashing Algorithm

## Beyond the Standard Collections



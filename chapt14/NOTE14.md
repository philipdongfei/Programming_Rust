# Closures

Other examples of standard library features that accept closures include:

- **Iterator** methods such as *map* and *filter*, for working with sequential data.
- Threading APIs like **thread::spawn**, which starts a new system thread. Concurrency is all about moving work to other threads, and closures conveniently represent units of work.
- Some methods that conditionally need to compute a default value, like the *or_insert_with* method of *HashMap* entries.

## Capturing Variables

A closure can use data that belongs to an enclosing function.

### Closures That Borrow

    /// Sort by any of several different statistics.
    fn sort_by_statistic(cities: &mut Vec<City>, stat: Statistic) {
        cities.sort_by_key(|city| -city.get_statistic(stat));
    }

In this case, when Rust creates the closure, it automatically borrows a reference to *stat*. It stands to reason: the closure refers to *stat*, so it must have a reference to it.
In short, Rust ensures safety by using lifetimes instead of garbage collection.

### Closures That Steal

The solution to both problems is the same: tell Rust to **move** *cities* and *stat* into the closures that use them instead of borrowing references to them.

    use std::thread;
    fn start_sorting_thread(mut cities: Vec<City>, stat: Statistic)
        -> thread::JoinHandle<Vec<City>>
        {
            let key_fn = move |city: &City| -> i64 { -city.get_statistic(stat) }; // key_fn, takes ownership of stat
            thread::spawn(move || {
                cities.sort_by_key(key_fn);
                cities
            })
        }

Note that **||** is the closure's empty argument list.
The only thing we've changed is to add the **move** keyword before each of the two closures. The **move** keyword tells Rust that a closure doesn't borrow the variables it uses: it steals them.

Rust thus offers two ways for closures to get data from enclosing scopes: moves and borrowing.
- Just as everywhere else in the language, if a closure would **move** a value of a copyable type, like i32, it copies the value instead. So if Statistic happened to be a copyable type, we could keep using stat even after creating a **move** closure that uses it.
- Values of noncopyable types, like *Vec<City>*, really are moved: the preceding code transfers *cities* to the new thread, by way of the **move** closure. Rust would not let us access *cities* by name after creating the closure.
- As it happens, this code doesn't need to use *cities* after the point where the closure moves it. If we did, though, the workaround would be easy: we could tell Rust to clone *cities* and store the copy in a different variable. The closure would only steal one of the copies--whichever one it refers to.

We get something important by accepting Rust's strict rules: thread safety.


## Function and Closure Types

## Closure Performance

## Closures and Safety

## Callbacks

## Using Closures Effectively



# Concurrency

Approaches that systems programmers commonly use include the following:

* A *background thread* that has a single job and periodically wakes up to do it.
* General-purpose *worker pools* that communicate with clients via *task queues*.
* *Pipelines* where data flows from one thread to the next, with each thread doing a little of the work.
* *Data parallelism*, where it is assumed (rightly or wrongly) that the whole computer will mainly just be doing one large computation, which is therefore split into *n* pieces and run on *n* threads in the hopes of putting all *n* of the machine's cores to work at once.
* *A sea of synchronized objects*, where multiple threads have access to the same data, and races are avoided using ad hoc *locking* schemes based on low-level primitives like mutexes.
* *Atomic integer operations* allow multiple cores to communicate by passing information through fields the size of one machine word.


## Fork-Join Parallelism

This pattern is called *fork-join parallelism*. To *fork* is to start a new thread, and to *join* a thread is to wait for it to finish.
Fork-join parallelism is attractive for a few reasons:

* It's dead simple.
* It avoids bottlenecks.
* The performance math is staightforward.
* It's easy to reason about program correctness.


### spawn and join

Here's a more substantial example, using *spawn* to implement a parallel version of the **process_file** function from before:

    use std::{thread, io};
    fn process_files_in_parallel(filenames: Vec<String>) -> io::Result<()> {
        // Divide the work into serveral chunks.
        const NTHREADS: usize = 8;
        let worklists = split_vec_into_chunks(filenames, NTHREADS);

        // Fork: Spawn a thread to handle each chunk.
        let mut thread_handles = vec![];
        for worklist in worklists {
            thread_handles.push(
                thread::spawn(move || process_files(worklist))
            );
        }

        // Join: Wait for all threads to finish.
        for handle in thread_handles {
            handle.join().unwrap()?;
        }

        Ok(())

    }

Note how we get the list of filenames into the worker thread:

* **worklist** is defined and populated by the for loop, in the parent thread.
* As soon as the **move** closure is created, **worklist** is moved into the closure.
* **spawn** then moves the closure (including the **worklist** vector) over to the new child thread.


### Error Handling Across Threads

Let's revisit that line of code:

    handle.join().unwrap()?;

The **.join()** method does two neat things for us.

First, **handle.join()** returns a **std::thread::Result** that's an error *if the child thread panicked*.
Second, **handle.join()** passes the return value from the child thread back to the parent thread.
In Rust, errors are **REsult** values (data) instead of exceptions (control flow). They're delivered across threads just like any other value.Any time you use low-level threading APIs, you end up having to write careful errorhandling code, but *given that you have to write it*, **Result** is very nice to have around.


### Sharing Immutable Data Across

Fortunately, the standard library provides another way: atomic reference counting.

    use std::sync::Arc;

    fn process_files_in_parallel(filenames: Vec<String>,
                                glossary: Arc<GigabyteMap>)
        -> io::Result<()>
        {
            ...
            for worklist in worklists {
                // This call to .clone() only clones the Arc and bumps the
                // reference count. It does not clone the GigabyteMap.
                let glossary_for_child = glossary.clone();
                thread_handles.push(
                    spawn(move || process_files(worklist, &glossary_for_child))
                );
            }
            ...
        }

With this change, the program compiles and runs, because it no longer depends on reference lifetimes. As long as *any* thread owns an **Arc<GigabyteMap>**, it will keep the map alive, even if the parent thread bails out early. There won't be any data races, because data in an **Arc** is immutable.


### Rayon

The Rayon library is another example. It provides two ways of running tasks concurrently:

    use rayon::prelude::*;

    // "do 2 things in parallel"
    let (v1, v2) = rayon::join(fn1, fn2);

    // "do N things in parallel"
    giant_vector.par_iter().for_each(|value| {
        do_thing_with_value(value);
    });



Here's a version of **process_files_in_parallel** using Rayon and a **process_file** that takes, rather than **Vec<String>**, just a **&str**:

    use rayon::prelude::*;

    fn process_files_in_paralle(filenames: Vec<String>, glossary: &GigabyteMap) -> io::Result<()>
    {
        filenames.par_iter()
            .map(|filename| process_file(filename, glossary))
            .reduce_with(|r1, r2| {
                if r1.is_err() { r1 } else { r2 }
            })
            .unwrap_or(Ok(()))
    }



### Revisiting the Mandelbrot Set

This is easy to fix using Rayon. We can just fire off a parallel task for each row of pixels in the output. This creates several hundred tasks that Rayon can distribute across its threads. Thanks to work-stealing, it won't matter that the tasks vary in size. Rayon will balance the work as it goes.


## Channels

A *channel* is a one-way conduit for sending values from one thread to another. In other words, it's a thread-safe queue.
They're something like Unix pipes: one end is for sending data, and the other is for receiving. The two ends are typically owned by two different threads. But whereas Unix pipes are for sending bytes, channels are for sending Rust values.
Rust channels are faster than Unix pipes. Sending a value moves it rather than copying it, and moves are fast even when you're moving data structures that contain many megabytes of data.

### Sending Values

### Receiving Values

### Running the Pipeline

Pipelines are like assembly lines in a manufacturing plant: performance is limited by the throughput of the slowest stage.

### Channel Features and Performance

The **mpsc** part of **std::sync::mpsc** stands for *multiproducer, single-consumer*, a terse description of the kind of communication Rust's channels provide.

**Sender<T>** implements the **Clone** trait. To get a channel with multiple senders, simply create a regular channel and clone the sender as many times as you like. You can move each **Sender** value to a different thread.
A **Receiver<T>** can't be cloned, so if you need to have multiple threads receiving values from the same channel, you need a **Mutex**.

Here Rust again takes a page from Unix pipes. Unix uses an elegant trick to provide some *backpressure* so that fast senders are forced to slow down: each pipe on a Unix system has a fix size, and if a process tries to write to a pipe that's momentarily full, the system simply blocks that process until there's room in the pipe. The Rust equivalent is called a *synchronous channel*.
A synchronous channel is exactly like a regular channel except that when you create it, you specify how many values it can hold. For a synchronous channel, **sender.send(value)** is potentially a blocking operation. After all, the idea is that blocking is not always bad.


### Thread Safety: Send and Sync

This is mostly true, but Rust's full thread safety story hinges on two built-in traits, **std::marker::Send** and **std::marker::Sync**.

* Types that implement **Send** are safe to pass by value to another thread. They can be moved across threads.
* Types that implement **Sync** are safe to pass by non-mut reference to another thread. They can be shared across threads.

By *safe* here, we mean the same thing we always mean: free from data races and other undefined behavior.


### Piping Almost Any Iterator to a Channel

This is Rust's character in a nutshell: we're free to add a concurrency power tool to almost every iterator in the language--but not without first understanding and documenting the restrictions that make it safe to use.

Traits allow us to add methods to standard library types, so we can actually do this. We start by writing a trait that declares the method we want:

    use std::sync::mpsc;

    pub trait OffThreadExt: Iterator {
        /// Transform this iterator into an off-thread iterator: the
        /// `next()` calls happen on a separate worker thread, so the
        /// iterator and the body of your loop run concurrently.
        fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
    }

Then we implement this trait for iterator types. It helps that **mpsc::Receiver** is already iterable:

    use std::thread;

    impl<T> OffThreadExt for T
        where T: Iterator + Send + 'static,
        T::Item: Send + 'static
    {
        fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
            // Create a channel to transfer items from the worker thread.
            let (sender, receiver) = mpsc::sync_channel(1024);

            // Move this iterator to a new worker thread and run it there.
            thread::spawn(move || {
                for item in self {
                    if sender.send(item).is_err() {
                        break;
                    }
                }
            });

            // Return an iterator that pulls values from the channel.
            receiver.into_iter()
        }
    }


### Beyond Pipelines

In this section, we used pipelines as our examples because pipelines are a nice, obvious way to use channels. Everyone understands them. They're concrete, practical, and deterministic. Channels are useful for more than just piplines, though. They're also a quick, easy way to offer any asynchronous service to other threads in the same process.

The tools we've presented so far--fork-join for highly parallel computation, channels for loosely connecting components--are sufficient for a wide range of applications.

## Shared Mutable State
 
The problem to be solved here is that many threads need access to a shared list of players who are waiting to join a game. This data is necessarily both mutable and shared across all threads.
You could solve this by creating a new thread whose whole job is to manage this list. Other threads would communicate with it via channels.
Another option is to use the tools Rust provides for safely sharing mutable data.

### What Is a Mutex?

A *mutex* (or *lock*) is used to force multiple threads to take turns when accessing certain data.

Mutexes are helpful for serveral reasons:
* They prevent *data races*, situations where racing threads concurrently read and write the same memory.
* Even if data races didn't exist, even if all reads and writes happened one by one in program order, without a mutex the actions of different threads could interleave in arbitrary ways.
* Mutexes support programming with *invariants*, rules about the protected data that are true by construction when you set it up and maintained by every critical section.

### Mutex<T>

The guard even lets us borrow direct references to the underlying data. Rust's lifetime system ensures those references can't outlive the guard itself. There is no way to access the data in a **Mutex** without holding the lock.
When **guard** is dropped, the lock is released. Ordinarily that happens at the end of the block, but you can also drop it manually.

### mut and Mutex

In Rust **&mut** means *exclusive access*. Plain **&** means *shared access*.
But **Mutex** does have a way: the lock. In fact, a mutex is little more than a way to do exactly this, to provide *exclusive* (mut) access to the data inside, even though many threads may have *shared* (non-mut) access to the **Mutex** itself.
Rust's type system is telling us what **Mutex** does. It dynamically enforces exclusive access, something that's usually done statically, at compile time, by the Rust compiler.


### Why Mutexes Are Not Always a Good Idea

However, threads that use mutexes are subject to some other problems that Rust doesn't fix for you:

* Valid Rust programs can't have data races, but they can still have other *race conditions*--situations where a program's behavior depends on timing among threads and may therefore vary from run to run.
* Shared mutable state also affects program design. Where channels serve as an abstraction boundary in your code, making it easy to separate isolated components for testing, mutexes encourage a "just-add-a-method" way of working that can lead to a monolithic blob of interrelated code.
* Lastly, mutexes are just not as simple as they seem at first, as the next two sections will show.

All of these problems are inherent in the tools. Use a more structured approach when you can; use a Mutex when you must.


### Deadlock

To put it another way, the lock in a **Mutex** is not a recursive lock.
Rust's borrow system can't protect you from deadlock. The best protection is to keep critical sections small: get in, do your work, and get out.
It's also possible to get deadlock with channels. In a pipeline, like out inverted index builder, data flow is acyclic.

### Poisoned Mutexes

If a thread panics while holding a **Mutex**, Rust marks the **Mutex** as *poisoned*. Any subsequent attempt to **lock** the poisoned **Mutex** will get an error result.
Rust poisons the mutex to prevent other threads from blundering unwittingly into this broken situation and make it worse. You *can* still lock a poisoned mutex and access the data inside, with mutual exclusion fully enforced; But you won't do it by accident.

### Multiconsumer Channels Using Mutexes

We can add a **Mutex** around the **Receiver** and share it anyway.

    pub mod shared_channel {
        use std::sync::{Arc, Mutex};
        use std::sync::mpsc::{channel, Sender, Receiver};

        /// A thread-safe wrapper around a `Receiver`.
        #[derive(Clone)]
        pub struct SharedReceiver<T>(Arc<Mutex<Receiver<T>>>);

        impl<T> Iterator for SharedReceiver<T> {
            type Item = T;

            /// Get the next item from the wrapped receiver.
            fn next(&mutself) -> Option<T> {
                let guard = self.0.lock().unwrap();
                guard.recv().ok()
            }
        }

        /// Create a new channel whose receiver can be shared across threads
        /// This returns a sender and a receiver, just like the stdlib's
        /// `channel()`, and sometimes works as a drop-in replacement.
        pub fn shared_channel<T>() -> (Sender<T>, SharedReceiver<T>) {
            let (sender, receiver) = channel();
            (sender, SharedReceiver(Arc::new(Mutex::new(receiver))))
        }
    }



### Read/Write Locks (RwLock<T>)

Whereas a mutex has a single **lock** method, a read/write lock has two locking methods, **read** and **write**. The **RwLock::write** method is like **Mutex::lock**. It waits for exclusive, **mut** access to the protected data. The **RwLock::read** method provides non-**mut** access, with the advantage that it is less likely to have to wait, because many threads can safely read at once. With a mutex, at any given moment, the protected data has only one reader or writer (or none). With a read/write lock, it can have either one writer or many readers, much like Rust references generally.
Rust, of course, is uniquely well suited to enforce the safety rules on **RwLock** data. The single-writer-or_multiple-reader concept is the core of Rust's borrow system.

### Condition Variables (Condvar)

Often a thread needs to wait until a certain condition becomes true:

* During server shutdown, the main thread may need to wait until all other threads are finished exiting.
* When a worker thread has nothing to do, it needs to wait until there is some data to process.
* A thread implementing a distributed consensus protocol may need to wait until a quorum of peers have responded.

### Atomics

The **std::sync::atomic** module contains atomic types for lock-free concurrent programming. These types are basically the same as Standard C++ atomics, with some extras:

* **AtomicIsize** and **AtomicUsize** are shared integer types corresponding to the single-threaded **isize** and **usize** types.
* **AtomicI8**, **AtomicI16**, **AtomicI32**, **AtomicI64**, and their unsigned variants like **AtomicU8** are shared integer types that correspond to the single-threaded types i8, i16, etc.
* An **AtomicBool** is a shared bool value.
* An **AtomicPtr<T>** is a shared value of the unsafe pointer type *mut T.

Of course, there are other ways to implement this. The **AtomicBool** here could be replaced with a **Mutex<bool>** or a channel. The main difference is that atomics have minimal overhead. Atomic operations never use system calls. A load or store often compiles to a single CPU instruction.



### Global Variables

Atomic globals are limited to simple integers and Booleans. Still, creating a global variable of any other type amounts to solving two problems.
First, the variable must be made thread-safe somehow, because otherwise it can't be global: for safety, static Variables must be both **Sync** and non-**mut**. Fortunately, we've already seen the solution for this problem. Rust has types for safely sharing values that change: **Mutex**, **RwLock**, and the atomic types. These types can be modified even when declared as non-**mut**.
Second, static initializers can only call functions specifically marked as **const**, which the compiler can evaluate during compile time. Put another way, their output is deterministic; it depends only on their arguments, not any other state or I/O. That way, compiler can embed the results of that computation as a compile-time constant.
You can also define your own **const** functions by simply prefixing the function's signature with **const**. Rust limits what **const** functions can do to a small set of operations, which are enough to be useful while still not allowing any nondeterministic results. **const** functions can't take types as generic arguments, only lifetimes, and it's not possible to allocate memory or operate on raw pointers, even in **unsafe** blocks. We can, however, use arithmetic operations (including
wrapping and saturating arithmetic), logical operations that don't short-circuit, and other **const** functions.


## What Hacking Concurrent Code in Rust Is Like


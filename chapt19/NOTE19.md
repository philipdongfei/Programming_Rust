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

## Shared Mutable State

## What Hacking Concurrent Code in Rust Is Like

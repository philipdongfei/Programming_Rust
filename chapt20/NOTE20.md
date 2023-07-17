# Asynchronous Programming

You can use Rust *asynchronous tasks* to interleave many independent activities on a single thread or a pool of worker threads. Asynchronous tasks are similar to threads, but are much quicker to create, pass control amongst themselves more efficiently, and have memory overhead an order of magnitude less than that of a thread.


## From Synchronous to Asynchronous

While this function is waiting for the system calls to return, its single thread is blocked: it can't do anything else until the sytem call finishes.

### Futures

### Async Functions and Await Expressions

### Calling Async Functions from Synchronous Code: block_on

### Spawning Async Tasks

### Async Blocks

### Building Async Functions from Async Blocks

### Spawning Async Tasks on a Thread Pool

### But Does Your Future Implement Send?

### Long Running Computations: yield_now and spawn_blocking


### Comparing Asynchronous Designs

### A Real Asynchronous HTTP Client

## An Asynchronous Client and Server

## Primitive Futures and Executors: When Is a Future Worth Polling Again?

## Pinning

## When Is Asynchronous Code Helpful?



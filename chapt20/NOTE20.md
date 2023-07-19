# Asynchronous Programming

You can use Rust *asynchronous tasks* to interleave many independent activities on a single thread or a pool of worker threads. Asynchronous tasks are similar to threads, but are much quicker to create, pass control amongst themselves more efficiently, and have memory overhead an order of magnitude less than that of a thread.


## From Synchronous to Asynchronous

While this function is waiting for the system calls to return, its single thread is blocked: it can't do anything else until the sytem call finishes.

### Futures

Rust's approach to supporting asynchronous operations is to introduce a trait, **std::future::Future**:

    trait Future { 
        type Output;
        // For now, read `Pin<&mut Self>` as `&mut Self`.
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) 
                -> Poll<Self::Output>;
    }

    enum Poll<T> {
        Ready(T),
        Pending,
    }

A **Future** represents an operation that you can test for completion. A future's poll method never waits for the operation to finish: it always returns immediately. If the operation is complete, **poll** return **Poll::Ready(output)**, where **output** is its final result. Otherwise, it returns **Pending**. If and when the future is worth polling again, it promises to let us know by invoking a *waker*, a callback function supplied in the **Context**.

So an asynchronous version of **read_to_string** would have a signature roughly like this: 

    fn read_to_string(&mut self, buf: &mut String)
        -> impl Future<Output = Result<usize>>;

    fn read_to_string<'a>(&'a mut self, buf: &'a mut String)
        -> impl Future<Output = Result<usize>> + 'a;

One of the rules of the **Future** trait is that, once a future has returned **Poll::Ready**, it may assume it will never be polled again. Some futures just return **Poll::Pending** forever if they are overpolled; others may panic or hang.


### Async Functions and Await Expressions


    use async_std::io::prelude::*;
    use async_std::net;
    
    async fn cheapo_request(host: &str, port: u16, path: &str)
                                -> std::io::Result<String>
    {
        let mut socket = net::TcpStream::connect((host, port)).await?;
    
        let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
        socket.write_all(request.as_bytes()).await?;
        socket.shutdown(net::Shutdown::Write)?;
    
        let mut response = String::new();
        socket.read_to_string(&mut response).await?;
    
        Ok(response)
    }

This is token for token the same as our original version, except:

* The function starts with **async fn** instead of **fn**.
* It uses the **async_std** crate's asynchronous versions of **TcpStream::connect**, **write_all**, and **read_to_string**. These all return futures of their results. 
* After each call that returns a future, the code says **.await**. Although this looks like a reference to a struct field named **await**, it is actually special syntax built into the language for waiting until a future is ready. An **await** expression evaluates to the final value of the future. This is how the function obtains the results from **connect**, **write_all**, and **read_to_string**.


### Calling Async Functions from Synchronous Code: block_on

We can call **cheapo_request** from an ordinary, synchronous function (like main, for example) using **async_std's task::block_on** function, which takes a future and polls it until it produces a value:


    fn main() -> std::io::Result<()> {
        use async_std::task;
    
        let response = task::block_on(cheapo_request("www.rust-lang.com", 80, "/"))?;
        println!("{}", response);
        Ok(())
    }

Since **block_on** is a synchronous function that produces the final value of an asynchronous function, you can think of it as an adapter from the asynchronous world to the synchronous world. But its blocking character also means that you should never use **block_on** within an async function: it would block the entire thread until the value is ready. Use **await** instead.


### Spawning Async Tasks

the goal of this chapter is to get the thread *doing other work* while it's waiting.
For this, you can use **async_std::task::spawn_local**. This function takes a future and adds it to a pool that **block_on** will try polling whenever the future it's blocking on isn't ready.
The **spawn_local** function is an asynchronous analogue of the standard library's **std::thread::spawn** function for starting threads:

* **std::thread::spawn(c)** takes a closure **c** and starts a thread running it, returning a **std::thread::JoinHandle** whose **join** method waits for the thread to finish and returns whatever **c** returned.
* **async_std::task::spawn_local(f)** takes the future **f** and adds it to the pool to be polled when the current thread calls **block_on.spawn_local** returns its own **async_std::task::JoinHandle** type, itself a future that you can await to retrieve **f'**s final value.

We have finally achieved the goal we set out at the beginning of the chapter: letting a thread take on other work while it waits for I/O to complete so that the thread's resources aren't tied up doing nothing. Even better, this goal was met with code that looks very much like ordinary Rust code: some of the functions are marked **async**, some of the function calls are followed by **.await**, and we use functions from **async_std** instead fo **std**, but otherwise,
it's ordinary Rust code.
One important difference to keep in mind between asynchronous tasks and threads is that switching from one async task to another happens only at **await** expressions, when the future being awaited returns **Poll::Pending**. This means that if you put a long-running computation in **cheapo_request**, none of the other tasks you passed to **spawn_local** will get a chance to run until it's done. With threads, this problem doesn't arise: the operating system can suspend
any thread at any point and sets timers to ensure that no thread monopolizes the processor.

### Async Blocks

In addition to asynchronous functions, Rust also supports *asynchronous blocks*. Whereas an ordinary block statement returns the value of its last expression, an async block returns *a future of* the value its last expression. You can use **await** expressions within an async block.


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



# Input and Output

Rust's standard library features for input and output are organized around three traits, **Read**, **BufRead**, and **Write**:

* Values that implement **Read** have methods for byte-oriented input. They're called *readers*.
* Values that implement **BufRead** are *buffered* readers. They support all the methods of **Read**, plus methods for reading lines of text and so forth.
* Values that implement **Write** support both byte-oriented and UTF-8 text output. They're called *writers*.



## Readers and Writes

*Readers* are values that your program can read bytes from.

* Files opened using **std::fs::File::open(filename)**
* **std::net::TcpStreams**, for receiving data over the network
* **std::io::stdin()**, for reading from the process's standard input stream
* **std::io::Cursor<&[u8]>** and **std::io::Cursor<Vec<u8>>** values, which are readers that "read" from a byte array or vector that's already in memory

*Writers* are values that your program can write bytes to.

* Files opened using **std::fs::File::create(filename)**
* **std::net::TcpStreams**, for sending data over the network
* **std::io::stdout()** and **std::io::stderr()**, for writing to the terminal
* **Vec<u8>**, a writer whose **write** methods append to the vector
* **std::io::Cursor<Vec<u8>>**, which is similar but lets you both read and write data, and seek to different positions within the vector
* **std::io::Cursor<&mut [u8]>**, which is much like **std::io::Cursor<Vec<u8>>**, except that it can't grow the buffer, since it's just a slice of some existing byte array

### Readers

**std::io::Read** has serveral methods for reading data.

**reader.read(&mut buffer)**
**reader.read_to_end(&mut byte_vec)**
**reader.read_to_string(&mut string)**
**reader.read_exact(&mut buf)**
**reader.bytes()**
**reader.chain(reader2)**
**reader.take(n)**

There is no method for closing a reader. Readers and writers typically implement **Drop** so that they are closed automatically.


### Buffered Readers

For efficiency, readers and writers can be *buffered*, which simply means they have a chunk of memory (a buffer) that holds some input or output data in memory. 

**reader.read_line(&mut line)**
**reader.lines()**
**reader.read_until(stop_byte, &mut byte_vec)**, **reader.split(stop_byte)**



### Reading Lines

Note that a **File** is not automatically buffered. **File** implements **Read** but not **BufRead**. However, it's easy to create a buffered reader for a **File**, or any other unbuffereD reader. **BufReader::new(reader)** does this. (To set the size of the buffer, use **BufReader::with_capacity(size, reader)**.)

### Collecting Lines

The standard library contains an implementation of **FromIterator** for **Result**--easy to overlook in the online documentation--that makes this possible:

    impl<T, E, C> FromIterator<Result<T, E>> for Result<C, E>
        where C: FromIterator<T>
        {
            ...
        }

This requires some careful reading, but it's a nice trick. Assume C is any collection type, like Vec or HashSet. As long we already know how to build a C from an iterator of T values, we can build a **Result<C, E>** from an iterator producing **Result<T, E>** values. We just need to draw values from the iterator and build the collection from the Ok results, but if we ever see an Err, stop and pass that along.
In other words, **io::Result<Vec<String>>** is a collection type, so the **.collect()** method can create and populate values of that type.


### Writers

As we've seen, input is mostly done using methods. Output is a bit different.

To send output to a writer, use the **write!()** and **writeln!()** macros. They are the same as **print!()** and **println!()**, except for two differences:

    writeln!(io::stderr(), "error: world not helloable")?;
    writeln!(&mut byte_vec, "The greatest common divisor of {:?} is {}",
        numbers, d)?;

One difference is that the **write** macros each take an extra first argument, a writer. The other is that they return a **Result**, so errors must be handled. That's why we used the ? operator at the end of each line.
The **print** macros don't return a **Result**; they simply panic if the write fails. Since they write to the terminal, this is rare.

**writer.write(&buf)**
**writer.write_all(&buf)**
**writer.flush()**
    Flushes any buffered data to the underlying stream. Returns **Result<()>**.
    Note that while the **println!** and **eprintln!** macros automatically flush the stdout and stderr stream, the **print!** and **eprintln!** macros automatically flush the stdout andstderr stream, the **print!** and **eprint!** macros do not. You may have to call **flush()** manually when using them.

Like readers, writers are closed automatically when they are dropped.
When a **BufWriter** is dropped, all remaining buffered data is written to the underlying writer. However, if an error occurs during this write, the error is *ignored*. (Since this happens inside **BufWriter's .drop()** method, there is no useful place to report the error.) To make sure your application notices all output errors, manually **.flush()** buffered writers before dropping them.

### Files

We've already seen two ways to open a file:

**File::open(filename)**
**File::create(filename)**

Note that the File type is in the filesystem module, **std::fs**, not **std::io**.
Once a **File** has been opened, it behaves like any other reader or writer. You can add a buffer if needed. The **File** will be closed automatically when you drop it.

### Seeking

**File** also iimplements the **Seek** trait, which means you can hop around within a **File** rather than reading or writing in a single pass from the beginning to the end.

    pub trait Seek {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64>;
    }
    
    pub enum SeekFrom {
        Start(u64),
        End(i64), 
        Current(i64)
    }

Seeking within a file is slow.

### Other Reader and Writer Types

**io::stdin()**
**io::stdout(), io::stderr()**
**Vec<u8>**
**Cursor::new(buf)**
    Creates a **Cursor**, a buffered reader that reads from **buf**. This is how you create a reader that reads from a **String**.
    Using a cursor to write past the end of a **Vec<u8>** is fine, though: it grows the vector.
**std::net::TcpStream**
**std::process::Command**
**io::sink()**
**io::empty()**
**io::repeat(byte)**

### Binary Data, Compression, and Serialization

The **byteorder** crate offers **ReadBytesExt** and **WriteBytesExt** traits that add methods to all readers and writers for binary input and output:
The **flate2** crate provides adapter methods for reading and writing **gzip**ped data
The **serde** crate, and its associated format crates such as **serde_json**, implement serialization and deserialization: they convert back and forth between Rust structs and bytes.


## Files and Directories


### OsStr and Path

**OsStr** is a string type that's a superset of UTF-8. Its job is to be able to represent all filenames, command-line arguments, and environment variables on the current system, *whether they're valid Unicode or not*. 
So we have two string types: **str** for actual Unicode strings; and **OsStr** for whatever nonsense your operating sytem can dish out.
Lastly, for each string types, there's a corresponding *owning* type: a **String** owns a heap-allocated **str**, a **Std::ffi::OsString** owns a heap-allocated **OsStr**, and a **std::path::PathBuf** owns a heap-allocated **Path**.


### Path and PathBuf Methods

### Filesystem Access Functions

### Reading Directories

### Platform-Specific Features


## Networking


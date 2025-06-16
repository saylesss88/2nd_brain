---
id: struct_Buffer
aliases: []
tags: []
---

# Struct Buffer

**Buffers in Rust**: in Rust, a buffer is typically a block of memory used for
temporary storage of data. Buffers are commonly used when reading or writing
data to or from sources like files, network sockets, or memory. They help
optimize I/O operations by reducing the number of system calls or memory
allocations.

```rust
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
  let file_path = "example.txt";
  let mut buffer = [0u8; 1024]; // A buffer with a capacity of 1024 bytes

  let mut file = File::open(file_path)?; // Returns Err if file not there

  let bytes_read = file.read(&mut buffer)?;

  // Now, 'buffer' contains the data read from the file.

  println!("Read {} bytes from the file.", bytes_read);

  Ok(())  // Explicit success return
}
```

- `File::open`: Returns `io::Result<File>`.

    - `?` unwraps `Ok(file)` or returns `Err` from main.

- `let bytes_read = file.read(&mut buffer)?;`:

    - Reads up to 1024 bytes from `file` into `buffer`.

    - Returns `io::Result<usize>`:

        - `Ok(n)`: Number of bytes read (0 <= `n` <= 1024).

        - `Err(e)`: I/O error.

    - `?` propagates any error or assigns `n` to `bytes_read`.

```rust
pub struct Buffer {
  data: Arc<Bytes>,
  ptr: *const u8,
  length: usize,
}
```

A contiguous memory region that can be shared with other buffers and across
thread boundaries that stores Arrow data.

`Buffer`s can be sliced and cloned without copying the underlying data nad can
be created from memory allocated by non-Rust sources such as C/C++.

Create a `Buffer` from a `Vec` (without copying):

```rust
let vec: Vec<u32> = vec![1, 2, 3];
let buffer = Buffer::from(vec);
```

Convert a `Buffer` to a `Vec` (without copying):

```rust
// convert the buffer back into a Vec of u32
// this will fail if the buffer is shared or not aligned correctly
let vec: Vec<u32> = buffer.into_vec().unwrap();
```

Create a `Buffer` from a [`bytes::Bytes`] (without copying):

```rust
let bytes = bytes::Bytes::from("hello");
let buffer = Buffer::from(bytes);
```

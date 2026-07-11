# std::collections

## Vectors

A contiguous growable array type, written as `Vec<T>`, short for `vector`.

The capacity of a vector is the amount of space allocated for any future
elements that will be added to the vector. This is not to be confused with the
_length_ of a vector, which specifies the number of actual elements within the
vector. If a vectors length exceeds its capacity, its capacity will
automatically be increased, but its elements will have to be reallocated.

```rs
let mut vec = Vec::new();
vec.push(1);
vec.push(2);

assert_eq!(vec.len(), 2);
assert_eq!(vec[0], 1);

assert_eq!(vec.pop(), Some(2));
assert_eq!(vec.len(), 1);

vec[0] = 7;
assert_eq!(vec[0], 7);

vec.extend([1,2,3]);

for x in &vec {
  println!("{x}");
}

assert_eq!(vec, [7, 1, 2, 3]);
```

## vec! macro

If `vec!` contains no elements it uses `Vec::new()`, if there are items in it,
it uses `with_capacity`.

The `vec!` macro is provided for convenient initialization:

```rs
let mut vec1 = vec![1, 2, 3];
vec1.push(4);
let vec2 = Vec::from([1, 2, 3, 4]);
assert_eq!(vec1, vec2);

// It can also initialize each element of a `Vec<T>` with a given value.
// This may be more efficient than performing allocation and initialization
// in separate steps, especially when initializing a vector of zeros:
let vec = vec![0; 5];
assert_eq!(vec, [0, 0, 0, 0, 0]);

// The following is equivalent, but potentially slower:
let mut vec = Vec::with_capacity(5);
vec.resize(5, 0);
assert_eq!(vec, [0, 0, 0, 0, 0]);
```

Use a `Vec<T>` as an efficient stack:

```rs
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    // Prints 3, 2, 1
    println!("{top}");
}
```

### struct Vec

```rs
pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    buf: RawVec<T, A>,
    len: usize,
}
```

A `RawVec` represents a vector of a certain size.

Vectors impement `IntoIterator` but not the `Iterator` trait because vectors
aren't iterators. Iterators need an index value keeping track of where they are
in the iterator, vectors only have a pointer, length, and capacity.

## VecDeque

A double-ended queue implemented with a growable ring buffer.

A ring buffer is a vector with 2 pointers to start and end.

```text
|-------------------|
|        |C(len)    |
|--------|----------|

|-------------------|
|  |start      |end |
|--------|----------|
   ^ 2 pointers^
# Anything between the pointers is initialized memory
# When you push onto end, start stays where it is and
# The end gets added to until it wraps around to the left of start
# The ring buffer is full when end = start (len == capacity)
```

A queue is FIFO first-in-first-out you push to one end of the collection and pop
off the other.

You cannot turn a `VecDeque` into a slice since they're not necessarilly
contiguous. There is a method `make_contiguous` that rearranges the storage
so it is contiguous.


### Resources

- [Module collections](https://doc.rust-lang.org/std/collections/index.html)
- [Struct Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [Rustonomicon Implementing Vec](https://doc.rust-lang.org/nomicon/vec/vec.html)
- [std lib alloc/src/raw_vec/mod.rs](https://github.com/rust-lang/rust/blob/main/library/alloc/src/raw_vec/mod.rs)
- [RawVec Rustonomicon](https://doc.rust-lang.org/nomicon/vec/vec-raw.html)

Struct `RawVec`:

```rs
struct RawVecInner<A: Allocator = Global> {
    ptr: Unique<u8>,
    /// Never used for ZSTs; it's `capacity()`'s responsibility to return usize::MAX in that case.
    ///
    /// # Safety
    ///
    /// `cap` must be in the `0..=isize::MAX` range.
    cap: Cap,
    alloc: A,
}
```

Vectors deref to slices.

When you have a vector you can use it as a slice and access any methods
available for slices.

- [rust-lang](https://github.com/rust-lang/rust/tree/master/library/alloc)

- [Rust reference ABI](https://doc.rust-lang.org/reference/abi.html)

- [What is an ABI (Application Binary Interface)](https://slightknack.github.io/rust-abi-wiki/intro/what_is_an_ABI.html)

An ABI, is the public-facing API of an executable that determines how other
programs can call into it.

ABIs describe two main facilities:

- How data is laid out in memory.
- How functions are called.

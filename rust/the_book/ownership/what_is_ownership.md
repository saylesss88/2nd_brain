# What Is Ownership

_Ownership_ is a set of rules that govern how a Rust program manages memory.

In Rust memory is managed through a system of ownership with a set of rules that
the compiler checks.

## The Stack and the Heap

Both the stack and the heap are parts of memory available to your code to use at
runtime, but they are structured in different ways.

## The Stack

- The stack stores values in the order it gets them and removes them in the
  opposite order. This is called _last in, first out (LIFO)_.
  - Adding data is called _pushing onto the stack_, and removing data is called
    _popping off the stack_.
  - All data stored on the stack must have a known, fixed size. Data with an
    unknown size at compile time or a size that might change must be stored on
    the heap instead.

### The Heap

When you put data on the heap, you request a certain amount of space. The memory
allocator finds an empty spot in the heap that's big enough, marks it as being
in use, and returns a pointer, which is the address location. This process is
called _allocating on the heap_.

Because the pointer to the heap is a known, fixed size, you can store the
pointer on the stack, but when you want the actual data, you have to follow the
pointer.

The main purpose of ownership is to manage heap data.

## Ownership Rules

- Each value in Rust has an _owner_.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Variable Scope

As a first example of ownership, we’ll look at the scope of some variables. A
scope is the range within a program for which an item is valid. Take the
following variable:

```rust
let s = "hello";
```

The variable `s` refers to a string literal, where the value of the string is
hardcoded into the text of our program. The variable is valid from the point at
which it's declared until the end of the current scope.

```rust
{                   // s isn't valid, since it's undeclared
  let s = "hello";  // s is valid from this point forward

                    // do stuff with s
}                   // this scope is now over, and s is no longer valid
```

There are two important points in time here:

- When `s` comes _into_ scope, it is valid.
- It remains valid until it goes _out_ of scope.

### The String Type

The `String` type manages data allocated on the heap and as such is able to
store an amount of text that is unknown to us as compile time.

You can create a `String` from a string literal using the `from` function:

```rust
let s = String::from("hello");
```

The double colon `::` operator allows us to namespace this particular `from`
function under the `String` type rather than using some sort of name like
`string_from`.

```rs
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `hello, world!`
```

Why can `String` be mutated but literals cannot? The difference is in how these
two types deal with memory.

## Memory and Allocation

In the case of a string literal, we know the contents at compile time, so the
text is hardcoded directly into the final executable. This is why string
literals are fast and efficient. But these properties only come from the string
literal's immutability.

With the String type, in order to support a mutable, growable piece of text, we
need to allocate an amount of memory on the heap, unknown at compile time, to
hold the contents. This means:

- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we’re done with
  our String.

That first part is done by us: When we call `String::from`, its implementation
requests the memory it needs. This is pretty much universal in programming
languages.

The second part is different. The memory is automatically returned once the
variable that owns it goes out of scope.

```rs
{
  let s = String::from("hello");  // s is valid from this point forward

  // do stuff with s
}                                 // this scope is now over, and s is no
                                  // longer valid
```

When a variable goes out of scope, Rust calls a special function for us.
The function is called `drop`, and it's where the author of `String` can put
the code to return the memory. Rust calls `drop` automatically at the closing
curly bracket.

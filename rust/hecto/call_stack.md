---
id: call_stack
aliases: []
tags: []
---

# What is the Call Stack

- The call stack is a data structure (a stack) that keeps track of active
  function calls in a program. Each time a function is called, a new "frame" is
  pushed onto the stack; when the function returns, its frame is popped off.

- **Order**: It represents the hierarchy and sequence of function invocations,
  from the entry point (`main`) down to the currently executing function.

## How it Works

1. **Function Call**: When a function is invoked, a stack frame is created
   containing:

    - The function's arguments.

    - Local variables.

    - The return address (where to go back to when the function finishes).

2. **Stack Growth**: As functions call other functions, the stack grows downward
   (new frames are added).

3. **Return**: When a function completes, its frame is removed, and control
   returns to the caller.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<String, io::Error> {
  let mut file = File::open(path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  Ok(contents)
}

fn count_lines(path: &str) -> Result<usize, io::Error> {
  let contents = read_file(path)?;
  let line_count = contents.lines().count();
  Ok(line_count)
}

fn main() {
  match count_lines("example.txt") {
    Ok(count) => println!("Number of lines: {}", count),
    Err(e) => println!("Error: {}", e),
  }
}
```

**Call Stack Walkthrough**

Assume `main` calls `count_lines`, which calls `read_file`. Here's how the call
stack evolves:

1. Program Starts:

- Stack: `[main]`

- `main` is the entry point.

2. `main` calls `count_lines`:

- Stack: `[main, count_lines]`

- `count_lines("example.txt")` is invoked, pushing its frame onto the stack.

3. `count_lines` calls `read_file`:

- Stack: `[main, count_lines, read_file]`

- `read_file("example.txt")` is called, adding another frame.

4. `read_file` executes:

- Inside read_file, File::open and read_to_string are called, but these are
  library functions. Their internal calls (e.g., system calls) might add more
  frames temporarily, but we’ll focus on our code.

- If `File::open` succeeds, execution continues. If it fails, the `?` operator
  triggers an early return.

5. Error Case with `?`:

- If `File::open` returns `Err`, the `?` in `read_file` returns that `Err`
  immediately.

- Stack unwinds: `[main, count_lines, read_file]` -> `[main, count_lines]`.

- Then `count_lines`'s `?` propagates the `Err`, unwinding to `[main]`.

- `main` handles it in the `match`.

6. Success Case:

- If all operations succeed, `read_file` returns `Ok(contents)`.

- Stack: `[main, count_lines]` (pops `read_file`).

- `count_lines` computes the line count and returns `Ok(count)`.

- Stack: `[main]` (pops `count_lines`).

- `main` prints the result.

## Buffer Injection

Old pattern:

```rs
impl ListCmd {
    /// Parses the index and prints a formatted list of available sprites.
    pub fn run(&self) -> Result<()> {
        let content = std::fs::read_to_string(&self.index_path)?;
        let entries: Vec<px2ansi::indexer::ImageEntry> = serde_json::from_str(&content)?;
        let limit = self.count.unwrap_or(entries.len()).min(entries.len());
        println!(
            "{} Showing {} of {} entries:",
            "Index:".magenta().bold(),
            limit,
            entries.len()
        );
        for entry in entries.iter().take(limit) {
            println!(
                "  • {:<20} {}x{}px",
                entry.name.cyan(),
                entry.dimensions.0.to_string().dimmed(),
                entry.dimensions.1.to_string().dimmed()
            );
        }
        Ok(())
    }
}
```

We were using `println!` or writing directly to `std::io::stdout()` inside each
command.

- How it worked: Every time your code called println!, it had to ask the
  Operating System for permission to write to the terminal.

- The Problem: For a tool like px2ansi-rs, which generates thousands of tiny
  ANSI escape codes for colors and characters, this meant thousands of syscalls.

- The Bottleneck: Syscalls are expensive. The CPU spends more time "context
  switching" (talking to the OS) than actually rendering your pixel art.

## Using BufWriter

New pattern:

```rs
use std::{io::Write, path::PathBuf};


impl ListCmd {
       pub fn run<W: Write>(&self, writer: &mut W) -> Result<()> {
       ┊   let content: String = std::fs::read_to_string(&self.index_path)?;
       ┊   let entries: Vec<px2ansi::indexer::ImageEntry> = serde_json::from_str(&c
       ┊
       ┊   let limit: usize = self.count.unwrap_or(default: entries.len()).min(entr
       ┊
       ┊   writeln!(
       ┊   ┊   writer,
       ┊   ┊   "{} Showing {} of {} entries:",
       ┊   ┊   "Index:".magenta().bold(),
       ┊   ┊   limit,
       ┊   ┊   entries.len()
       ┊   )?;
       ┊
       ┊   for entry: &ImageEntry in entries.iter().take(limit) {
       ┊   ┊   writeln!(
       ┊   ┊   ┊   writer,
       ┊   ┊   ┊   "  • {:<20} {}x{}px",
       ┊   ┊   ┊   entry.name.cyan(),
       ┊   ┊   ┊   entry.dimensions.0.to_string().dimmed(),
       ┊   ┊   ┊   entry.dimensions.1.to_string().dimmed()
       ┊   ┊   )?;
       ┊   }
       ┊   Ok(())
       } fn run
   } impl
```

By introducing `BufWriter` and "injected" writers, we've optimized the process
in three ways:

1. The Power of the Buffer

Instead of thousands of tiny writes, BufWriter collects those small pieces of
data in a 128KB memory buffer (the size you set in main.rs). Once the buffer is
full, it performs one large write to the OS. This reduces the number of syscalls
by several orders of magnitude.

2. Locking Stdout

In Rust, io::stdout() is protected by a mutex because multiple threads might try
to write to it. By using .lock(), you take the lock once at the start of the
program and hold it, rather than the program constantly "locking and unlocking"
for every single line.

3. Dependancy Injection (The `run<W: Write>` change)

This was the biggest architectural change. By making your commands accept a
generic writer, you decoupled the **Logic** from the **Output**.

- Before: `ListCmd` was "hardcoded" to talk to the terminal.

- After: `ListCmd` just says, "I will write to anything that supports the Write
  trait." This makes your code more "testable" and flexible.

**Summary of Changes**



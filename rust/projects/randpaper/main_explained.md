# main.rs explained

```rs
#![allow(clippy::multiple_crate_versions)]
mod cli;
mod core;
mod hyprland;
mod sway;

use clap::Parser;

use crate::cli::{Backend, Cli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.backend {
        Backend::Sway => sway::run_sway_loop(cli).await,
        Backend::Hyprpaper => hyprland::run_hyprpaper_loop(cli).await,
    }
}
```

1. The Async Entry Point: `#[tokio::main]`

By default, Rust's main function is synchronous. It runs one line at a time and
blocks the thread while waiting for things like I/O.

Because you are likely dealing with event loops (like run_sway_loop), you need
Concurrency. The #[tokio::main] macro does some "magic" behind the scenes:

- It transforms your `async fn main()` into a standard synchronous `main()`.

- It starts a Multi-threaded Runtime (the "Executor").

- It essentially wraps your code in a block that says: "Start the engine, and
  don't stop until this `main` block finishes."

---

2. Error Handling: `anyhow::Result<()>`

In a CLI app, you usually don't want to write complex error types for every
possible failure (file not found, IPC socket closed, etc.).

- The `Result` Return: By returning `anyhow::Result<()>`, your main function can
  now return any error that implements the standard Error trait.

- Simplifying the Match: Notice that you aren't using `unwrap()` or `expect()`.
  If `run_sway_loop` returns an Err, main will simply pass that error up, print
  it to the console, and exit with a non-zero status code.

- The `?` Operator: Though not in your snippet, inside your loop functions, you
  can now use `?` on almost any operation, and `anyhow` will "catch" it.

---

3. The Match Statement as a Dispatcher

Your match `cli.backend` is the Traffic Controller of your crate.

Because you've defined `Backend` as an enum in `cli.rs`, Rust guarantees
Exhaustive Checking. If you decide to support a third compositor (like
`Backend::Wayfire`) and add it to your enum, the compiler will literally refuse
to build the project until you add a third arm to this match statement.

```Rust
match cli.backend {
    // If the user typed "--backend sway", this arm fires
    Backend::Sway => sway::run_sway_loop(cli).await,

    // If the user typed "--backend hyprpaper", this arm fires
    Backend::Hyprpaper => hyprland::run_hyprpaper_loop(cli).await,
}
```

---

How it all flows together Cli::parse(): clap looks at the command line arguments
and builds your Cli struct.

The Match: It looks at the backend field and decides which module (sway or
hyprland) should take control.

The .await: Since your loop functions are async, the match arm hands control
over to Tokio. The program "pauses" at that line while the loop runs, but the
CPU is free to handle other background tasks if you had them.

The Result: When the loop eventually finishes (or crashes), the result is passed
back to main, and anyhow ensures the exit is graceful.

A Quick Observation You are passing the entire cli struct into your sub-modules
(e.g., sway::run_sway_loop(cli)). This is great because those modules now have
access to all the user's flags (like verbosity or refresh rates) defined in your
Cli struct.

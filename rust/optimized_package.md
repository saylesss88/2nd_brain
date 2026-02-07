## Optimize Rust Crate

1. Check binary size:

```bash
# Build release binary
cargo build --release

# Check size (Linux/macOS)
ls -lh target/release/snowflake-bounce

# Or use du for human-readable
du -h target/release/snowflake-bounce
```

2. Compare before/after:

```bash
# Before optimization (if you still have the old binary)
ls -lh target/release/snowflake-bounce

# After - rebuild with new settings
cargo clean
cargo build --release
ls -lh target/release/snowflake-bounce
```

3. Check what cargo will publish:

```bash
# See exactly what files will be included
cargo package --list

# Check the package size
cargo package --dry-run
# Look for "Packaged X files, Y KiB" at the end
```

4. Analyze binary contents:

```bash
# Install cargo-bloat to see what's using space
cargo install cargo-bloat

# See what's taking up space in your binary
cargo bloat --release

# Top 10 functions by size
cargo bloat --release -n 10
```

5. Check stripped vs unstripped:

```bash
# With strip = true in Cargo.toml
cargo build --release
ls -lh target/release/snowflake-bounce

# Without strip (temporarily remove from Cargo.toml)
cargo build --release
ls -lh target/release/snowflake-bounce
```

Expected results with your optimizations:

- Debug build: 5-10 MB

- Release without optimizations: 2-3 MB

- Release with your optimizations: 300-600 KB

Run `cargo build --release && ls -lh target/release/snowflake-bounce` to see
your actual size! 🎯

## Slim down png files

```bash
cargo install oxipng
oxipng -o 4 -i 0 -r assets/images/
```

Explanation of flags:

- -o 4: High optimization level.

- -i 0: Remove interlacing (saves bytes).

- -r: Recursive (process all files in the folder).

Check Real Size:

```bash
cargo package
ls -lh target/package/slasher-horrorscripts-0.1.2.crate
```

**Timings**

- "s" = `cargo build --release  89.73s user 9.71s system 311% cpu 31.969 total`

- "z" = `cargo build --release  80.16s user 9.54s system 307% cpu 29.131 total`

- 3 = `cargo build --release  100.21s user 9.78s system 294% cpu 37.313 total`

- 3 with `lto = "fat"` =
  `cargo build --release  98.83s user 9.79s system 293% cpu 37.014 total`

**Size**

- "s" = `.rwxr-x---@ 2.0M jr    3 Feb 12:36  󰡯 target/release/persway`

- "z" = `.rwxr-x---@ 1.9M jr    3 Feb 12:32  󰡯 target/release/persway`

- 3 = `.rwxr-x---@ 2.6M jr    3 Feb 12:41  󰡯 target/release/persway`

- 3 with `lto = "fat"` =
  `.rwxr-x---@ 2.6M jr    3 Feb 12:47  󰡯 target/release/persway`

**Conclusion**

Use these for production:

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"
debug = false
```

## Cargo Bloat

```bash
 cargo bloat --release
    Finished `release` profile [optimized] target(s) in 0.10s
    Analyzing target/release/persway

 File  .text    Size          Crate Name
 1.7%   4.0% 81.5KiB regex_automata regex_automata::meta::strategy::new
 1.0%   2.3% 46.3KiB    serde_json? <&mut serde_json::de::Deserializer<R> as serde_core::de::Des...
 0.9%   2.0% 41.6KiB        persway persway::server::daemon::Daemon::run::{{closure}}
 0.6%   1.4% 27.8KiB  swayipc_types swayipc_types::error::event::<impl swayipc_types::reply::Eve...
 0.5%   1.1% 22.7KiB   clap_builder clap_builder::parser::parser::Parser::get_matches_with
 0.5%   1.0% 21.3KiB       persway? <persway::Args as clap_builder::derive::CommandFactory>::com...
 0.4%   1.0% 20.7KiB regex_automata regex_automata::nfa::thompson::compiler::Compiler::c
 0.4%   0.9% 18.5KiB swayipc_types? <swayipc_types::reply::_::<impl serde_core::de::Deserialize ...
 0.4%   0.9% 17.5KiB            std std::backtrace_rs::symbolize::gimli::Cache::with_global
 0.4%   0.8% 17.3KiB   clap_builder clap_builder::parser::validator::Validator::validate
 0.4%   0.8% 17.3KiB swayipc_types? <swayipc_types::reply::_::<impl serde_core::de::Deserialize ...
 0.4%   0.8% 16.9KiB regex_automata regex_automata::meta::regex::Builder::build
 0.3%   0.8% 16.4KiB  swayipc_async swayipc_async::socket::spawn::{{closure}}
 0.3%   0.7% 14.3KiB          tokio tokio::runtime::builder::Builder::build
 0.3%   0.7% 14.3KiB        persway persway::main
 0.3%   0.6% 13.2KiB regex_automata regex_automata::nfa::thompson::compiler::Compiler::compile
 0.3%   0.6% 12.1KiB            std std::backtrace_rs::symbolize::gimli::Context::new
 0.3%   0.6% 11.8KiB        persway persway::utils::relayout_workspace::{{closure}}
 0.2%   0.5% 10.9KiB   regex_syntax <regex_syntax::hir::translate::TranslatorI as regex_syntax::...
 0.2%   0.5% 10.8KiB regex_automata regex_automata::nfa::thompson::pikevm::PikeVM::search_imp
33.7%  76.9%  1.5MiB                And 2739 smaller methods. Use -n N to show more.
43.8% 100.0%  2.0MiB                .text section size, the file size is 4.6MiB
```

Everything here looks standard.

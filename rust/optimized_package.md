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

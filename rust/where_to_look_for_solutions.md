## Quick Reference Card

1. 1st — **The compiler error itself**

```bash
rustc --explain E0XXX
```

Run that with whatever error code it threw. Half the time it has an example of
exactly your situation.

2. 2nd — **Standard library docs** `doc.rust-lang.org/std` — search the type
   you're working with. `String`, `Vec`, `Option`, whatever. The method list +
   examples on each type page answers most "how do I do X" questions.

3. 3rd — Rust by Example `doc.rust-lang.org/rust-by-example` — code first,
   explanation second. Much better than the book for "just show me it working."

4. 4th — **Search with the right keywords**

Always include `rust` + the exact type name + what you want to do:

- `rust &str to String`
- `rust Vec iterate`
- `rust match destructure tuple`

Never vague. Type names are the key.

5. 5th — **Stack Overflow** / `users.rust-lang.org` SO for common stuff, the
   official forum for weirder things. Both are high quality for Rust
   specifically.

6. 6th — **The book** Honestly use it last as a reference, not first. You
   already know where things roughly live now.

7. 100th — **Grep real source code**

`grep -r "pattern" ~/.cargo/registry/src` — search through crates you already
have downloaded locally. Seeing how published crates actually use something
beats any explanation.

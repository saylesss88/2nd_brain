# Dispatch and Fat Pointers

- _Monomorphization_ is the process of turning generic code into specific code
  by filling in the concrete types that are used when compiled.

```rs
fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

// This does the same thing
fn strlen2<S>(s: S) -> usize
where
    S: AsRef<str>,
{
  s.as_ref().len()
}

fn foo() {
  strlen("hello world");  // &'static str
  strlen(String::from("hei verden")); // String: AsRef<str>
}
```

The compiler will generate functions for each of the `strlen` calls, something
like:


Both are generic functions that can take in any type that can be turned into a
reference to a `str`.

- [Trait AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html). Used to
  do a cheap reference-to-reference conversion.

<details>
<summary> Trait AsRef </summary>

```rs
pub trait AsRef<T>where
    // `?Sized` means relax the requirement that this must be `Sized`
    T: ?Sized,{
    // Required method
    fn as_ref(&self) -> &T;
}
```

In Rust, every generic type parameter has an _implicit_ `T: Sized` bound by
default. You don't see it, but it's always there unless you opt out. So
normally:

```rs
fn foo<T>(x: T) { ... }
// is actually
fn foo<T: Sized>(x: T) { ... }
```

`Sized` means "we know this types size at compile time".

</details>

After calling `strlen` twice, the compiler monomorphizes the generics into
non-generics:

```rs
pub fn strlen_refstr(s: &str) -> usize {
  s.len()
}
pub fn strlen_string(s: String) -> usize {
  s.len()
}

pub fn bool_then<T>(b: bool, f: impl FnOnce() -> T) -> Option<T> {
  if b {
    Some(f())
  } else {
    None
  }
}
```

- _Monomorphization_ is great because you could end up producing much more
  efficient code.
  - It does also make the binary bigger because the compiler has to generate
    these copies. Slightly worse cache efficiency

- If you ever ran `bool_then` with a particular closure or function, it would
  generate a copy of `bool_then` with the function or closure directly in the
  functions `Some(f())`. This is _inlining_ which the compiler can choose to do
  if it wishes.

- Only the parts of the stdlib that you actually use are included with the
  binary.

## Dispatch

```rs
pub trait Hei {
  fn hei(&self);
}

impl Hei for &str {
  fn hei(&self) {
    println!("hei {}", self);
  }
}

pub fn foo() {
  "J".hei();
}

pub fn bar(h: impl Hei) {
  h.hei();
}
// Is equivalent to
pub fn bar2<H: Hei>(h: H) {
  h.hei();
}

// Turns into something like:
// static dispatch
pub fn bar_str(h: &str) {
  h.hei();
}
```

For `bar()`, write whichever version is more clear to you, `bar` and `bar2` are
equivalent.

If you don't want Monomorphization and don't want multiple copies.

```rs
pub trait Hei {
  fn hei(&self);
}

impl Hei for &str {
  fn hei(&self) {
    println!("hei {}", self);
  }
}

pub fn foo() {
    for h in vec!["T", "Tom"] {
        h.hei();
    }

}

pub fn bar(h: impl Hei) {
  h.hei();
}
```

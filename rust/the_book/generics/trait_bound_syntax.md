---
id: Trait Bound Syntax
aliases: [generics]
---

# Trait Bound Syntax

The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form known as a _trait bound_; it looks like this:

```rust
pub fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}
``` 

- This longer form is equivalent to the previous example but is more verbose. We place trait bounds with the declaration of the generic type parameter after a colon and inside angle brackets.

We can have two parameters that implement `Summary`:

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

- Using `impl Trait` is appropriate if we want this function to allow `item1` and `item2` to have different types (as long as they both implement `Summary`).

To force both parameters to have the same type, we must use a trait bound, like this:

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

## Specifying Multiple Trait Bounds with the + Syntax

```rust
pub fn notify(item: &(impl Summary + Display)) {
```

The `+` syntax is also valid with trait bounds on generic types:

```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

- With the two trait bounds specified, the body of `notify` can call `summarize` and use `{}` to format `item`.


## Clearer Trait Bounds with `where` Clauses

instead of this:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

we can use a `where` clause like this:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
```

[[returning_types_that_implement_traits.md]]

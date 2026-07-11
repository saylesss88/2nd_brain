# Subtyping and Variance

- [lifetime-variance-example](https://github.com/sunshowers-code/lifetime-variance)

```rs
fn main() {
  let s = String::new();
  let x: &'static str = "hello world";
  let mut y = &s;
  y = x;
}
```

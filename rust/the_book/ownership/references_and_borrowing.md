# References and Borrowing

To avoid having to return the `String` to the calling function to be able
to use it, we can use references.

A reference is like a pointer in that it's an address we can follow to
access the data stored at that address; that data is owned by some other
variable. Unlike a pointer, a reference is guaranteed to point to a valid
value of a particular type for the life of that reference.

```rs
fn main() {
  let s1 = String::from("hello");

  let len = calculate_length(&s1);

  println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
  s.len()
}
```

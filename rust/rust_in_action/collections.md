# Collections

## Arrays

An _array_ is a tightly-packed collection of the same thing (type). You can
replace items in an array, but its size cannot change.

Creating arrays takes two forms. We can provide a comma-delimited list within
square brackets ([1,2,3]) or a _repeat expression_, where you give two values
delimited by a semicolon ([0;100]).

Defining arrays and iterating over their elements:

```rs
fn main() {
  let one             = [1,2,3];
  let two: [u8; 3]    = [1,2,3];
  let blank1          = [0; 3];
  let blank2: [u8; 3] = [0; 3];

  let arrays = [one, two, blank1, blank2];

  for a in &arrays {                     // iterates over each array
    print!("{:?}:", a);// `arrays` holds 4 arrays. Do the following to each one
    for n in a.iter() {  // inside loop 1, so each array it walks every element and prints `n + 10`.
      print!("\t{} + 10 = {}", n, n+10);
    }

    let mut sum = 0;
    for i in 0..a.len() { // also inside loop 1, walks the array again to compute the total.
      sum += a[i];
    }
    println!("\t({:?} = {})", a, sum");
  }
}
```

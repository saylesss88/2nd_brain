## The lines() method

`lines()` returns an iterator over `&str` slices, one per line.

```rs
fn main() {
  // <1> <2>
  let penguin_data = "\
  common name,length (cm)
  Little penguin,33
  Yellow-eyed penguin,65
  Fiordland penguin,60
  Invalid,data
  ";

  let records = penguin_data.lines();

  println!("{:?}", records);
```

**Output**:

```sh
Lines(Map { iter: SplitInclusive { 0: SplitInternal { start: 0, end: 109, matcher: CharSearcher { haystack: "common name,length (cm)\n  Little penguin,33\n  Yellow-eyed penguin,65\n  Fiordland penguin,60\n  Invalid,data\n  ", finger: 0, finger_back: 109, needle: '\n', utf8_size: 1, utf8_encoded: [10, 0, 0, 0] }, allow_trailing_empty: false, finished: false } } })
```

- `haystack: "common name,length (cm)\n  Little penguin,33\n..."` — your raw
  string, with \n where the line breaks are
- `needle: '\n'` — it's splitting on newline characters
- `start: 0, end: 109` — it hasn't consumed anything yet (`finger: 0`)

So `lines()` is essentially a `split('\n')` under the hood, just sitting there
waiting. The moment you call `.collect()`, `.for_each()`, a `for` loop, etc., it
walks through and hands you the slices one by one. That laziness is the point —
you can chain a dozen iterator adapters (`.filter()`, `.map()`, `.skip()`) and
none of them do any work until the terminal call at the end. The whole chain
runs in one pass.

This is just printing the iterator itself and only shows the iterator struct,
not its contents. To actually see the elements you need to collect or iterate:

```rs
// See all lines as a Vec
let records: Vec<&str> = penguin_data.lines().collect();
println!("{:?}", records);
// ["  common name,length (cm)", "  Little penguin,33", ...]

// Or iterate and print one per line
for line in penguin_data.lines() {
    println!("{:?}", line);
}
// "  common name,length (cm)"
// "  Little penguin,33"
// "  Yellow-eyed penguin,65"
// ...
```

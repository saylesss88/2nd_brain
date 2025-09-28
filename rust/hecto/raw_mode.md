---
id: raw_mode
aliases: []
tags: []
---

# raw_mode

```rust
use std::io::{self, Read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
        if c == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
    }
}
```

- `enable_raw_mode().unwrap();` enables raw mode

```rust
if c == 'q' {
  disable_raw_mode().unwrap();
  break;
}
```

- This exits and disables raw mode when `q` is pressed.

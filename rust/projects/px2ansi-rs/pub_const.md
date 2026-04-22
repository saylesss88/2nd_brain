## Literal or inline literal

You'll also hear:

- **magic literals** — slightly pejorative term for inline values that aren't
  self-explanatory (these are fine since the context makes them obvious)


```rs
fn kanji(&mut self) -> std::io::Result<()> {
    self.charset_colored(
        &[
            "\u{3000}", "一", "二", "十", "口", "日", "田", "目", "国", "風", "龍", "龘",
        ],
        true,
    )
}
```

## Named Constant or Symbolic Constant

- **extracted constant** — when you take an inline literal and pull it out into
  a named const

```rs
pub const CHINESE: &[&str] = &[
    "\u{3000}", "一", "二", "十", "人", "丁", "口", "王", "日", "木", "金", "華", "爱", "黑", "墨",
    "龍", "龘",
];

fn chinese(&mut self) -> std::io::Result<()> {
    self.charset_colored(CHINESE, true)
}
```

Practical recommendation If these palettes are stable named presets, use pub
const or pub static and keep the render methods tiny. If they are experimental
and likely to change per algorithm or per font, keeping them local in the method
is fine, but const still tends to read better.

For your crate, I’d do this:

```rust
pub const KANJI: &[&str] = &[
    "\u{3000}", "一", "二", "十", "口", "日", "田", "目", "国", "風", "龍", "龘",
];

pub const CHINESE: &[&str] = &[
    "\u{3000}", "一", "二", "十", "人", "丁", "口", "王", "日", "木", "金", "華", "爱", "黑", "墨",
    "龍", "龘",
];

fn kanji(&mut self) -> std::io::Result<()> {
    self.charset_colored(KANJI, true)
}

fn chinese(&mut self) -> std::io::Result<()> {
    self.charset_colored(CHINESE, true)
}
```

The real difference is then architectural: reusable palette constants vs
embedded literals. The only semantic bug to avoid is using `"\\u{3000}"` when
you actually mean the real U+3000 character.

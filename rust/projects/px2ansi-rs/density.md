```rs
RenderStylePreset::Dense => {
    opts.charset = CharsetMode::Ascii;
    opts.style.density = Density::Heavy;
}
```

Then in `render.rs` the ascii method checks density:

```rs
fn ascii(&mut self, density: Density) -> std::io::Result<()> {
    let charset: &[&str] = match density {
        Density::Light => &[
            " ", " ", ".", "`", "\"", "\\", ":", "I", "!", ">", "~", "_", "?", "[", "{",
            "|", ")", "(", "/", "Y", "L", "p", "d", "a", "*", "W", "8", "%", "@", "$",
        ],
        Density::Medium => &[
            " ", "`", ".", "-", "'", ":", "_", ",", "^", "=", ";", ">", "<", "+", "!", "r",
            "c", "*", "/", "z", "?", "s", "L", "T", "v", ")", "J", "7", "(", "|", "F", "i",
            "{", "C", "}", "f", "I", "3", "1", "t", "l", "u", "[", "n", "e", "o", "Z", "5",
            "Y", "x", "j", "y", "a", "]", "2", "E", "S", "w", "q", "k", "P", "6", "h", "9",
            "d", "4", "V", "p", "O", "G", "b", "U", "A", "K", "X", "H", "m", "8", "R", "D",
            "#", "$", "B", "g", "0", "M", "N", "W", "Q", "%", "&", "@",
        ],
        Density::Heavy => &[
            " ", ".", ":", ";", "i", "o", "x", "X", "O", "0", "#", "@", "█", "▓", "▒", "░",
            "█",
        ],
    };
    self.charset_colored(charset, false)
}
```

And pass density through from `write_ansi_art` in `render.rs`:

```rs
CharsetMode::Ascii => render.ascii(options.style.density),
```

That makes `--style ascii` use `Medium` by default, `--style dense` use `Heavy`,
and you could expose `--density light/medium/heavy` as a separate flag later if
you want fine control.

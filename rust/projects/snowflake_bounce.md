## Snowflake-Bounce Explained

**High-level behavior**

- Initializes an ncurses window and sets the locale so Unicode symbols
  (snowflakes, middle finger, etc.) render correctly.

- Uses `signal_hook` to flip atomic flags when resize/exit signals arrive.

- Runs a loop that:
  - Handles terminal resize and exit signals.

  - Reads keypresses (`q`, `c`, `s`, `f`) in non-blocking mode from pancurses.

  - Updates a `Bouncer` (position, velocity, color).

  - Draws a logo at the new position, bouncing off terminal edges.

---

`main.rs` explained

```rs
fn main() {
    // Enable UTF-8 locale for Unicode snowflakes
    unsafe {
        libc::setlocale(libc::LC_ALL, std::ffi::CString::new("").unwrap().as_ptr());
    }
```

- `setlocale(LC_ALL, "")` tells C/ncurses to use the user's environment locale,
  which is required for correct Unicode width/encoding.

```rs
let window = snowflake_bounce::ncurses_init();
```

- Calls your library helper to initialize ncurses and ruturns the main `Window`.

```rs
    let exit_signal = Arc::new(AtomicBool::new(false));
    let resize_signal = Arc::new(AtomicBool::new(false));

    flag::register(SIGWINCH, Arc::clone(&resize_signal)).unwrap();
    flag::register(SIGINT, Arc::clone(&exit_signal)).unwrap();
    flag::register(SIGTERM, Arc::clone(&exit_signal)).unwrap();
    flag::register(SIGQUIT, Arc::clone(&exit_signal)).unwrap();
```

- Creates shared `AtomicBools` that can be set from signal handlers.

- `signal_hook::flag::register` installs minimal signal handlers that set the
  given flag to true whenever the signal is delivered. ​

- `SIGWINCH` (window change) sets resize_signal, others set exit_signal.

```rs
loop {
  if resize_signal.swap(false, Ordering::Relaxed) {
    snowflake_bounce::resize_window();
    bouncer.resize();
  }
}
```

- `swap(false, Ordering::Relaxed)` returns the old value and resets the flag to
  `false`.

- If it was `true`, a resize signal happened: you reinitialize curses and let
  the `Bouncer` recompute bounds.

```rs
    if exit_signal.swap(false, Ordering::Relaxed) {
        snowflake_bounce::finish();
    }
```

- If an exit signal flag gets set, restore terminal state and exit.

```rs
    if let Some(Input::Character(c)) = window.getch() {
        match c {
            'q' => snowflake_bounce::finish(),
            'c' => bouncer.cycle_color(),
            's' => bouncer.cycle_symbol(),
            'f' => bouncer.set_middle_finger(),
            _ => {}
        }
    }
```

- window.getch() is non-blocking because of nodelay(true), so it returns None if
  no key is pressed.

- You match on characters:
  - `q` quits,

  - `c` randomizes color,

  - `s` cycles between symbol modes,

  - `f` switches to middle finger.

```rs
        bouncer.update();
        bouncer.draw(&window);
    }
}
```

- `update` moves the logo and handles bouncing/color changes.

- `draw` erases the old position and draws at the new one.

---

lib.rs core concepts

**RNG helper**

```rs
thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_entropy());
}

fn rng<T>() -> T
where
    Standard: Distribution<T>,
{
    RNG.with(|rng| (*rng).borrow_mut().r#gen::<T>())
}
```

- `thread_local!` gives each thread its own `SmallRng`.

- `RefCell` lets you mutably borrow the RNG even though it's stored in
  thread-local static.

- `rng::<T>()` uses `rand`’s `Standard` distribution to generate any type that
  has a default distribution (e.g., `bool`, `integers`).

**SymbolMode and Bouncer fields**

```rs
pub enum SymbolMode {
    SnowflakeSmall,
    SnowflakeLarge,
    NixOS,
    MiddleFinger,
}

pub struct Bouncer {
    x: i32,
    y: i32,
    prev_x: i32,
    prev_y: i32,
    dx: i32,
    dy: i32,
    color: i16,
    max_x: i32,
    max_y: i32,
    pub mode: SymbolMode,
}
```

- `mode` controls which "sprite" is drawn and what its logical width/height is

- `x, y` are the current top-left coordinates; `prev_x, prev_y` are used to
  erase the previous drawing region.

- `dx, dy` are step increments per frame.

- `max_x, max_y` are the effective terminal boundaries, used to bounce.

**Bouncer::new**

```rs
pub fn new() -> Self {
  let (max_y, max_x) = get_term_size();

  let start_x = rng::<i32>() % (max_x - 50).max(5) + 2;
  let start_y = rng::<i32>() % (max_y - 50).max(5) + 2;

  Self {
    x: start_x,
    y: start_y,
    prev_x: start_x,
    prev_y: start_y,
    dx: if rng::<bool>() { 1 } else { -1 },
    dy: if rng::<bool>() { 1 } else { -1 },
    color: COLOR_BLUE,
    max_x: max_x - 1,
    max_y: max_y - 1,
    mode: SymbolMode::NixOS,
  }
}
```

- `get_term_size()` returns `(rows, cols)` with minimum enforced dimensions
  using `term_size`.

- The random start position is padded away from edges to fit the NixOS logo.

- Velocity is randomly chosen ±1 in both axes.

**Mode and color handling**

```rs
    pub const fn cycle_symbol(&mut self) {
        self.mode = match self.mode {
            SymbolMode::SnowflakeSmall => SymbolMode::SnowflakeLarge,
            SymbolMode::SnowflakeLarge => SymbolMode::NixOS,
            SymbolMode::NixOS => SymbolMode::SnowflakeSmall,
            SymbolMode::MiddleFinger => SymbolMode::SnowflakeSmall,
        };
    }
```

- Simple state machine: press `s` to step through modes; `MiddleFinger` always
  jumps back to small snowflake when cycled.

```rs
    fn change_color(&mut self) {
        let colors = [
            COLOR_GREEN,
            COLOR_BLUE,
            COLOR_WHITE,
            COLOR_YELLOW,
            COLOR_CYAN,
            COLOR_MAGENTA,
            COLOR_RED,
        ];
        self.color = colors[rng::<usize>() % colors.len()];
    }
```

- Chooses a random color index from the list of ncurses colors; later used as
  the pair index in `COLOR_PAIR(self.color as chtype)`.

**Movement, bouncing, and resizing**

```rs
    pub fn update(&mut self) {
        self.prev_x = self.x;
        self.prev_y = self.y;

        self.x += self.dx;
        self.y += self.dy;

        let (logo_width, logo_height) = self.get_logo_dimensions();

        if self.x <= 0 {
            self.x = 0;
            self.dx = -self.dx;
            self.change_color();
        } else if self.x + logo_width >= self.max_x {
            self.x = self.max_x - logo_width;
            self.dx = -self.dx;
            self.change_color();
        }

        if self.y <= 0 {
            self.y = 0;
            self.dy = -self.dy;
            self.change_color();
        } else if self.y + logo_height >= self.max_y {
            self.y = self.max_y - logo_height;
            self.dy = -self.dy;
            self.change_color();
        }
    }
```

- Moves one step, then checks collisions against left/right and top/bottom
  boundaries.

- Uses logo width/height so the whole logo remains visible.

- On bounce, reverses direction and randomizes color

```rs
    pub fn resize(&mut self) {
        let (lines, cols) = get_term_size();
        self.max_y = lines - 1;
        self.max_x = cols - 1;

        let (logo_width, logo_height) = self.get_logo_dimensions();

        if self.x + logo_width >= self.max_x {
            self.x = (self.max_x - logo_width).max(0);
        }
        if self.y + logo_height >= self.max_y {
            self.y = (self.max_y - logo_height).max(0);
        }
    }
```

- Adjusts boundaries when terminal is resized.

- Clamps the position so the logo stays within visible area.

```rs
    const fn get_logo_dimensions(&self) -> (i32, i32) {
        match self.mode {
            SymbolMode::SnowflakeSmall => (1, 1),
            SymbolMode::SnowflakeLarge => (5, 3),
            SymbolMode::NixOS => (46, 19),
            SymbolMode::MiddleFinger => (2, 1),
        }
    }

    fn get_logo_lines(&self) -> Vec<&str> {
        match self.mode {
            SymbolMode::SnowflakeSmall => vec!["❄"],
            SymbolMode::SnowflakeLarge => vec!["  ❄  ", " ❄❄❄ ", "  ❄  "],
            SymbolMode::NixOS => vec![ /* big ASCII NixOS logo */ ],
            SymbolMode::MiddleFinger => vec!["🖕"],
        }
    }
```

- Width/height must match the ASCII art lines you provide; you use that for
  bounds and erase region.

- `get_logo_lines` returns the lines you actually draw.

**Drawing with pancurses**

```rs
    pub fn draw(&self, window: &Window) {
        let logo_lines = self.get_logo_lines();
        let (logo_width, logo_height) = self.get_logo_dimensions();

        for i in 0..logo_height {
            let erase_str = " ".repeat(logo_width as usize);
            window.mvaddstr(self.prev_y + i, self.prev_x, &erase_str);
        }

        window.attron(COLOR_PAIR(self.color as chtype));
        for (i, line) in logo_lines.iter().enumerate() {
            window.mvaddstr(
                self.y + i32::try_from(i).expect("logo line index too large"),
                self.x,
                line,
            );
        }
        window.attroff(COLOR_PAIR(self.color as chtype));

        window.refresh();
        napms(50);
    }
```

- Erases previous drawing rectangle by printing spaces across the width for each
  logo row.

- Sets a color pair with attron(COLOR_PAIR(...)).

- Draws each logo line at (y + row_index, x).

- Refreshes the window and sleeps 50 ms (napms) to control animation speed. ​

**Terminal helpers**

```rs
pub fn get_term_size() -> (i32, i32) {
    match term_size::dimensions() {
        Some((width, height)) => {
            let width = width.max(30);
            let height = height.max(15);
            (height as i32, width as i32)
        }
        None => (24, 80),
    }
}
```

- Uses term_size::dimensions() to get (width, height) and enforces minimums so
  the NixOS logo fits. ​

```rs
pub fn ncurses_init() -> Window {
    let window = initscr();

    window.nodelay(true);
    noecho();
    cbreak();
    curs_set(0);

    if has_colors() {
        start_color();
        use_default_colors();

        for i in 0..8 {
            init_pair(i, i, -1);
        }
    }

    window.refresh();
    window
}
```

- Standard ncurses initialization: raw-like input, no echo, hide cursor,
  non-blocking input, color support.

- Initializes 8 color pairs pair_number == foreground_color with default
  background.

```rs
pub fn resize_window() {
    endwin();
    initscr();
}

pub fn finish() {
    curs_set(1);
    endwin();
    std::process::exit(0);
}
```

- `resize_window` tears down and reinitializes ncurses to adapt to new terminal
  size. ​

- `finish` restores cursor visibility, ends ncurses mode, and exits.

## Adding multiple bouncing flakes

To do this, we need to make three major changes:

1. Container: Instead of one bouncer, we need a Vec<Bouncer> (a list of them).

2. Initialization: Change new() to accept a SymbolMode so you can spawn specific
   types (one small, one large).

3. Physics: Add a way to check if two bouncers occupy the same space and make
   them bounce.

Here is how to upgrade your code.

**Step 1**: Update `lib.rs` to handle Collisions

We need two new methods in your `Bouncer` struct: one to check if it hits
another bouncer, and one to "bounce" (reverse direction).

Add these methods to `impl Bouncer` in `src/lib.rs`:

```rust
impl Bouncer {
    // ... existing new, resize, update methods ...

    // 1. Allow creating a specific type of bouncer
    pub fn new_with_mode(mode: SymbolMode) -> Self {
        let mut b = Self::new();
        b.mode = mode;
        b
    }

    // 2. Simple Box Collision Detection (AABB)
    pub fn intersects(&self, other: &Bouncer) -> bool {
        let (w1, h1) = self.get_logo_dimensions();
        let (w2, h2) = other.get_logo_dimensions();

        // Convert u16 to i32 for safe comparison
        let x1 = self.x as i32;
        let y1 = self.y as i32;
        let x2 = other.x as i32;
        let y2 = other.y as i32;

        // Check if rectangles overlap
        x1 < x2 + w2 &&
        x1 + w1 > x2 &&
        y1 < y2 + h2 &&
        y1 + h1 > y2
    }

    // 3. Physics: Swap velocities implies an elastic collision
    pub fn bounce_off(&mut self, other: &mut Bouncer) {
        // Swap DX
        std::mem::swap(&mut self.dx, &mut other.dx);
        // Swap DY
        std::mem::swap(&mut self.dy, &mut other.dy);

        // Optional: Change colors when they hit!
        self.change_color();
        other.change_color();
    }
}
```

**Step 2**: Update `main.rs` to Run Multiple Flakes Now we need to update your
main loop to handle a list of bouncers instead of just one.

Replace your main function (or wherever your loop is) with this logic:

```rust
fn main() -> anyhow::Result<()> {
    // ... setup code (crossterm init, etc) ...

    // Create a list of bouncers
    let mut bouncers = vec![
        Bouncer::new_with_mode(SymbolMode::NixOS),          // The Big One
        Bouncer::new_with_mode(SymbolMode::SnowflakeSmall), // Small one
        Bouncer::new_with_mode(SymbolMode::SnowflakeLarge), // Medium one
    ];

    loop {
        // 1. Handle Input (Resize/Quit)
        if event::poll(std::time::Duration::from_millis(33))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('s') => {
                        // Resize/Change mode for ALL bouncers
                        for b in &mut bouncers {
                            b.cycle_symbol();
                        }
                    }
                    _ => {}
                }
            } else if let event::Event::Resize(w, h) = event::read()? {
                // Resize ALL bouncers
                for b in &mut bouncers {
                    b.resize(w, h);
                }
            }
        }

        // 2. Physics & Logic
        // Move everyone first
        for b in &mut bouncers {
            b.update();
        }

        // Check Collisions (Double loop)
        // We use indices to avoid borrowing issues
        for i in 0..bouncers.len() {
            for j in (i + 1)..bouncers.len() {
                // Check if bouncer[i] hits bouncer[j]
                // We have to split_at_mut to borrow both mutably at the same time
                let (slice_a, slice_b) = bouncers.split_at_mut(j);
                let b1 = &mut slice_a[i];
                let b2 = &mut slice_b[0]; // slice_b starts at index j

                if b1.intersects(b2) {
                    b1.bounce_off(b2);

                    // Hack: Move them 1 step to prevent sticking
                    b1.update();
                    b2.update();
                }
            }
        }

        // 3. Draw
        // Use a standard writer
        let mut stdout = io::stdout();
        for b in &bouncers {
            b.draw(&mut stdout)?;
        }
        stdout.flush()?;
    }

    // ... cleanup code ...
    Ok(())
}
```

Why this works

1. `intersects`: It treats every logo as a rectangle. If the rectangle of the
   NixOS logo overlaps with the Snowflake logo, it returns true.

2. `bounce_off`: When they hit, we swap their speeds (dx/dy). If the big one was
   moving Right (+1) and the small one Left (-1), after the hit, the big one
   moves Left (-1) and the small one Right (+1). It looks like a real bounce!

3. `split_at_mut`: Rust prevents you from borrowing bouncers[i] and bouncers[j]
   as mutable at the same time normally. split_at_mut is the safe, idiomatic way
   to say "Give me two separate mutable slices so I can touch two items at
   once."

**About the "Artifact" Bug**

Since you have multiple objects now, if they overlap, one might erase the other
when it moves.

- Simple fix: Just let it happen. It adds to the "glitchy" aesthetic you
  mentioned.

- Complex fix: You would need a "framebuffer" (a grid of characters in memory)
  where you draw everything first, then print the whole frame to the screen.
  That is significantly more complex. For now, try the method above—it should
  look chaotic and fun!

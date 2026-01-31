# Getting Started

```toml
[package]
name = "nix-snake"
version = "0.1.0"
edition = "2024"

[dependencies]
crossterm = "0.29.0"
rand = "0.8.5"
clap = "4.5.54"
```

When trying to figure out which modules/items to import, you typically start by
reading the crates documentation (`doc.rs`). You usually start by typing the
full path, like `crossterm::cursor::MoveTo`. Then your IDE will often suggest
"Import this..." and automatically add the `use` statement to the top for you.

- Don't worry about memorizing the imports; focus on the **concepts** (I need to
  move the cursor -> `cursor::MoveTo`), and the tooling handles the `use` lines.

## The "Game Loop" Architecture

Every game runs in an infinite loop that does three things over and over again,
usually 60 times a second:

1. **Input**: Did the user press a key?

2. **Update**: Move characters, calculate physics, check collisions.

3. **Draw**: Wipe the screen and paint the new state.

Your `main.rs` implements exactly this.

**Code Breakdown**

1. The Setup (Engine Initialization)

Before the loop starts, we have to take control of the terminal.

```rs
// Switch to "Alternate Screen" so we don't mess up the user's shell history.
// Hide the cursor so it doesn't blink annoyingly in the middle of our game.
execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

// Enable "Raw Mode".
// Normally, terminals wait for you to press ENTER before sending text.
// Raw mode gives us keypresses instantly (vital for games).
enable_raw_mode()?;
```

- To better understand **Canonical Mode vs Raw Mode**, I suggest reading
  [hecto-chapter-2](https://philippflenker.com/hecto-chapter-2/)

2. The Loop (The Heartbeat)

The `while running` loop is the game's heartbeat.

**Part A: Input Handling (Non-Blocking)**

This is the most critical part of a responsive game.

```rs
// poll() checks if there is an event WAITING to be read.
// We wait up to 100ms. If nothing happens, we continue anyway.
// This ensures the game keeps running (processing animations/physics)
// even if the player isn't pressing anything.
if event::poll(Duration::from_millis(100))? {
  // If we have an event, read it!
  let Event::Key(KeyEvent { code, .. }) = event::read()? {
    match code {
      KeyCode::Char('q') | KeyCode::Esc => running = false,
      _ => {}
    }
  }
}
// Clippy wants you to collapse the code as much as possible:
if event::poll(Duration::from_millis(100))?
    && let Event::Key(KeyEvent { code, .. }) = event::read()?
{
    match code {
        KeyCode::Char('q') | KeyCode::Esc => running = false,
        _ => {}
    }
}
```

- Why `poll`? If we just used `read()`, the program would FREEZE until you
  pressed a key. The game would stop. `poll` allows us to say "Check for keys,
  but if there aren't any, keep moving."

**Part B: Draw (Rendering)**

Terminal rendering works by painting over the old frame.

`crossterm` commands are just instructions. To send them to the terminal, you
have to "queue" them into the stdout buffer.

```rs
// 1. Wipe the slate clean.
execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

// 2. Move the "brush" (cursor) to x=10, y=10.
// 3. Set the ink color to Cyan.
// 4. Print the character.
execute!(
    stdout,
    cursor::MoveTo(10, 10),              // Command 1
    SetForegroundColor(Color::Cyan),     // Command 2
    Print("λ")
)?;                  // Automatically queues both and flushes
```

- `execute!` is syntactic sugar. It combines `queue!` and `flush!` into one
  line.

3. The Cleanup (Leaving Gracefully)

If we crash or exit without this, the user's terminal will be broken (cursor
hidden, typing invisible).

```rust
// Restore the cursor and switch back to the main shell screen.
execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
disable_raw_mode()?;
```

Key Concepts:

- **Raw Mode** vs. **Canonical Mode**: Why `std::io::stdin().read_line()`
  doesn't work for games (it waits for Enter).

- **The Frame Buffer**: We are essentially treating the terminal text grid as a
  pixel buffer (80x24 resolution).

<details>
<summary> ✔️ Frame Buffer Explained </summary>

**The "Pixel Buffer" Analogy**

When you play a modern video game (like Call of Duty), the computer has a grid
of millions of pixels (e.g., 1920x1080).

- **Buffer**: A chunk of memory that holds the color of every single pixel.

- **Drawing**: To draw a red square, the code calculates which pixels belong to
  that square and changes their color values in memory to "RED".

- **Refresh**: The monitor reads this memory 60 times a second and lights up the
  physical pixels.

**The Terminal "Grid"**

In a simple terminal game, you don't have millions of tiny pixels. You have a
much coarser grid, usually 80 columns by 24 rows (or larger).

- The "Pixel": Instead of a tiny dot of color, your "pixel" is a single
  character cell (like a box on graph paper).

- The "Color": Each cell holds two pieces of data:

1. **The Glyph**: What letter/symbol is it? (`'A'`, `'@'`, `'λ'`, `' '`)

2. **The Attribute**: What color is it? (Foreground Cyan, Background Black,
   Bold, etc.)

Why we call it a "Buffer" When you wrote:

```rust
execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
```

You are essentially "clearing the buffer" — wiping the entire grid clean, just
like `glClear()` in OpenGL clears the screen to black.

When you wrote:

```rust
execute!(stdout, cursor::MoveTo(10, 10), Print("λ"))?;
```

You are logically doing this:
`ScreenBuffer[10][10] = { char: 'λ', color: Cyan }`

</details>

<details>
<summary> ✔️ Game Loop Explained </summary>

1. Enter Alternate Screen: This gives you a fresh, blank canvas separate from
   the user's command history.

2. The Loop (Every Frame):

- **Wipe**: Erase everything (Clear screen).

- **Calculate**: Figure out where the snake should be now.

- **Draw**: Paint the snake at that new location.

- **Flush**: Show the result to the user.

**Wait, isn't that inefficient?**

Yes! Wiping the entire screen and redrawing every single character 10-60 times a
second sounds wasteful, and technically it is.

However:

1. **Terminals are fast**: Modern terminals (Alacritty, Kitty, even GNOME
   Terminal) can handle thousands of text updates per second easily.

2. **It's safer**: It guarantees your screen always looks exactly like your game
   state. You never end up with "ghost" snake segments left behind because you
   forgot to erase one specific spot.

**The "Optimization" (Optional Learning)**

In very advanced terminal engines (like `ratatui`), they do a "Diff."

1. They have a `Previous_Buffer` and a `Current_Buffer`.

2. They compare them. "Oh, only the snake's head moved from (10,10) to (11,10),
   and the tail moved from (5,10) to (6,10)."

3. They only send commands to update those specific cells.

But for a simple snake game, wipe & redraw is standard.

</details>

- **State Management**: The variable `running` is our first piece of "Game
  State". Later, `Snake` and `Food` structs will also live here, persisting
  between frames.

**Game Loop**

```rs
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use std::io::{stdout, Write};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // Setup
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut running = true;

    // Game Loop
    while running {
        // 1. Handle Input (just quitting for now)
        if event::poll(Duration::from_millis(100))? &&
            let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => running = false,
                    _ => {}
                }

        }

        // 2. Update

        // 3. Draw
        // Clear screen
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        execute!(
            stdout,
            cursor::MoveTo(10, 10),
            SetForegroundColor(Color::Cyan),
            Print("λ")
        )?;
    }
    // Cleanup
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
```

## The Walking Dot

Goal: Create a Snake struct and update its position in the loop so the lambda
travels across the screen.

Changes required:

Define the Snake: A struct to hold x, y (position) and dx, dy (direction).

Update Logic: Add x += dx and y += dy inside the loop.

Draw Logic: Use the snake's position in the execute! macro.

Timing: Add a thread::sleep so it doesn't zip across the screen in 1
millisecond.

Code:

Update your main.rs to look like this. I've marked the New sections.

```rust
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
    style::{Color, Print, SetForegroundColor},
};
use std::io::{stdout, Write};
use std::time::Duration;

// --- NEW: Define direction constants ---
const RIGHT: (i16, i16) = (1, 0);
const LEFT: (i16, i16) = (-1, 0);
const UP: (i16, i16) = (0, -1);
const DOWN: (i16, i16) = (0, 1);

// --- NEW: Snake Struct ---
struct Snake {
    // Position (using u16 because terminal coordinates are unsigned)
    // BUT we might use i16 for math to avoid underflow checks initially
    // Let's stick to u16 for pos and cast when moving.
    x: u16,
    y: u16,
    // Direction (dx, dy)
    dir: (i16, i16),
}

impl Snake {
    fn new() -> Self {
        Self {
            x: 10,
            y: 10,
            dir: RIGHT, // Start moving right
        }
    }

    fn update(&mut self) {
        // Simple move: cast to i16, add dir, cast back to u16
        // (We will add boundary checks next step, for now it might crash if it hits 0)
        self.x = (self.x as i16 + self.dir.0) as u16;
        self.y = (self.y as i16 + self.dir.1) as u16;
    }
}

fn main() -> std::io::Result<()> {
    // Setup
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    // --- NEW: Initialize Snake ---
    let mut snake = Snake::new();
    let mut running = true;

    // Game Loop
    while running {
        // 1. Handle Input (Non-blocking)
        // Check for events, but don't wait longer than 100ms (controls game speed!)
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => running = false,
                    // --- NEW: Manual Controls (Optional, for testing) ---
                    KeyCode::Left => snake.dir = LEFT,
                    KeyCode::Right => snake.dir = RIGHT,
                    KeyCode::Up => snake.dir = UP,
                    KeyCode::Down => snake.dir = DOWN,
                    _ => {}
                }
            }
        }

        // 2. Update
        snake.update();

        // 3. Draw
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        // Use snake.x and snake.y
        execute!(
            stdout,
            cursor::MoveTo(snake.x, snake.y),
            SetForegroundColor(Color::Cyan),
            Print("λ")
        )?;

        // --- NEW: Flush output explicitly ---
        stdout.flush()?;
    }

    // Cleanup
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
```

Try it out:

`cargo run`

Output:

```text
error[E0425]: cannot find value `snake` in this scope
  --> src/main.rs:60:38
   |
60 |                     KeyCode::Left => snake.dir = LEFT,
   |                                      ^^^^^ not found in this scope
```

We forgot to initialize snake! Add this right above `let mut running = true;`

```rs
let mut snake = Snake::new();
```

You should see the lambda moving right.

Test: Press arrow keys. Does it turn?

Warning: If you let it hit the wall, it might panic (because 0 - 1 wraps around
for u16 or crashes). We'll fix that next.

---

## Step 4: The Body (VecDeque) & Boundaries

Goal:

1. Change `Snake` to hold a list of positions (`body`), not just one `x`, `y`.

2. Make the snake "move" by adding a new head and removing the old tail.

3. Detect screen boundaries so it wraps around or resets instead of crashing.

Key Concept: The VecDeque Move

To move a snake efficiently:

- Step 1: Calculate new head position (`current_head + direction`).

- Step 2: `push_front(new_head)` into the deque.

- Step 3: `pop_back()` (remove the tail) so length stays constant (for now).

Code Changes:

Update Snake struct and methods in `main.rs`:

```rust
use std::collections::VecDeque; // <--- ADD THIS IMPORT

struct Snake {
    // OLD: x: u16, y: u16,
    // NEW: body holds all segments. body[0] is the head.
    body: VecDeque<(u16, u16)>,
    dir: (i16, i16),
}

impl Snake {
    fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_back((10, 10)); // Head
        body.push_back((9, 10));  // Tail segment 1
        body.push_back((8, 10));  // Tail segment 2 (start with length 3)

        Self {
            body,
            dir: RIGHT,
        }
    }

    fn update(&mut self) {
        // 1. Get current head
        let (head_x, head_y) = *self.body.front().expect("Snake has no body!");

        // 2. Calculate new head position
        // We cast to i16 to handle negative checking, then wrap/clamp
        let next_x = head_x as i16 + self.dir.0;
        let next_y = head_y as i16 + self.dir.1;

        // 3. Boundary Check (Simple Wrap-around for screensaver vibe)
        // If it goes off screen (approx 80x24 for now, we'll get real size later), wrap it.
        // Let's use a fixed size for this step to prevent crashes.
        let width = 80;
        let height = 24;

        let new_x = if next_x < 0 { width - 1 } else if next_x >= width { 0 } else { next_x as u16 };
        let new_y = if next_y < 0 { height - 1 } else if next_y >= height { 0 } else { next_y as u16 };

        // 4. Move: Add new head
        self.body.push_front((new_x as u16, new_y as u16));

        // 5. Remove tail (Simulate movement, not growing yet)
        self.body.pop_back();
    }
}
```

Update the Draw Loop in main: You now need to loop over the body to draw it.

```rust
        // 3. Draw
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

        // Draw the Snake
        for (i, point) in snake.body.iter().enumerate() {
            // Head gets the Lambda, body gets a different char (or fading color later)
            let symbol = if i == 0 { "λ" } else { "o" };
            execute!(
                stdout,
                cursor::MoveTo(point.0, point.1),
                SetForegroundColor(Color::Cyan),
                Print(symbol)
            )?;
        }

        stdout.flush()?;
```

Tasks:

Update the struct and new().

Update update() to use push_front/pop_back + wrapping logic.

Update the draw loop.

Run it! You should see a 3-segment snake (λoo) moving and wrapping around the
screen edges (at 80x24).

## Step 5: Dynamic Terminal Size & Food

Goal: Make the snake use the real terminal size and add things to eat.

1. Fix the Size: We need to ask crossterm for the actual terminal size on every
   update (or at least once).

Update the update method signature to accept the size:
`fn update(&mut self, width: u16, height: u16)`

2. Add Food: We need a Food struct and logic to check collisions.

Code Changes

1. Update Snake::update to take size:

```rust
    // In impl Snake
    fn update(&mut self, max_width: u16, max_height: u16) {
        let (head_x, head_y) = *self.body.front().expect("Snake has no body!");

        let next_x = head_x as i16 + self.dir.0;
        let next_y = head_y as i16 + self.dir.1;

        // Cast input dimensions to i16 for comparison
        let width = max_width as i16;
        let height = max_height as i16;

        let new_x: i16 = if next_x < 0 {
            width - 1
        } else if next_x >= width {
            0
        } else {
            next_x
        };

        let new_y: i16 = if next_y < 0 {
            height - 1
        } else if next_y >= height {
            0
        } else {
            next_y
        };

        self.body.push_front((new_x as u16, new_y as u16));
        self.body.pop_back();
    }
```

2. Pass the real size in main: Crossterm has a function terminal::size().

```rust
    // Inside the loop, right before snake.update()
    // We get the size every frame so if you resize the window, it adapts!
    let (term_cols, term_rows) = terminal::size()?;
    snake.update(term_cols, term_rows);
```

3. Define the Food Struct (New Struct):

```rust
use rand::Rng; // Add `rand` to imports

struct Food {
    x: u16,
    y: u16,
    symbol: char, // '❄' or '📦'
}

impl Food {
    // We need to know screen size to spawn randomly
    fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
            // Randomly pick a symbol
            symbol: if rng.gen_bool(0.5) { '❄' } else { '📦' },
        }
    }

    fn respawn(&mut self, width: u16, height: u16) {
        let mut rng = rand::thread_rng();
        self.x = rng.gen_range(0..width);
        self.y = rng.gen_range(0..height);
        self.symbol = if rng.gen_bool(0.5) { '❄' } else { '📦' };
    }
}
```

4. Integrate Food into main:

```rust
    // Before loop
    let (w, h) = terminal::size()?;
    let mut food = Food::new(w, h);

    // Inside loop:
    // 1. Draw Food
    execute!(
        stdout,
        cursor::MoveTo(food.x, food.y),
        SetForegroundColor(Color::Red),
        Print(food.symbol)
    )?;

    // 2. Check Collision (Eating)
    // We need to peek at the HEAD of the snake.
    if let Some(&(head_x, head_y)) = snake.body.front() {
        if head_x == food.x && head_y == food.y {
            // Respawn food
            let (w, h) = terminal::size()?;
            food.respawn(w, h);

            // GROW: To grow, we just skip the `pop_back` we did in update().
            // But since `update` does pop_back automatically, the easiest way
            // is to just add a dummy tail segment back, OR change `update` logic.

            // EASIER FIX: Let's change `Snake::update` to accept a `grow` boolean?
            // actually, let's just push a duplicate of the tail.
            if let Some(&tail) = snake.body.back() {
                snake.body.push_back(tail);
            }
        }
    }
```

Task:

Update snake.update to take width/height.

Pass terminal::size()? in the loop.

Add the Food struct and draw it.

Add the eating logic (grow the snake).

## Step 6: The Autopilot (AI)

Goal: Replace the arrow key controls with logic that automatically steers the
snake toward the food.

Simple AI Strategy (The "Greedy" Approach):

1. Look at the Food's position (food.x, food.y).

2. Look at the Head's position (head.x, head.y).

3. Move in a direction that reduces the distance (e.g., if food.x > head.x, go
   RIGHT).

4. Constraint: Don't turn 180 degrees instantly (can't go Left if currently
   going Right).

Limitations of Greedy AI:

It will get stuck in local traps (e.g., moving into a U-shaped body segment) or
hit walls if it blindly chases food without looking ahead. But it's a great
start for a screensaver!

Code Changes

1. Add an autopilot method to Snake:

```rust
    // inside impl Snake
    fn set_direction(&mut self, new_dir: (i16, i16)) {
        // Prevent 180 turns (banning reversing)
        // If current is RIGHT (1,0) and new is LEFT (-1,0), sum is (0,0).
        // This simple check works for opposite cardinal directions.
        if (self.dir.0 + new_dir.0 != 0) || (self.dir.1 + new_dir.1 != 0) {
            self.dir = new_dir;
        }
    }

    // The AI Logic
    fn autopilot(&mut self, food_x: u16, food_y: u16) {
        let (head_x, head_y) = *self.body.front().unwrap();

        // Determine ideal direction
        // Prioritize X movement first (arbitrary choice)
        if head_x < food_x && self.dir != LEFT {
            self.set_direction(RIGHT);
        } else if head_x > food_x && self.dir != RIGHT {
            self.set_direction(LEFT);
        } else if head_y < food_y && self.dir != UP {
            self.set_direction(DOWN);
        } else if head_y > food_y && self.dir != DOWN {
            self.set_direction(UP);
        }
        // Else: keep going current direction (or pick random safe turn if stuck)
    }
```

2. Update main loop:

Remove the manual match code (or keep it as an override) and call autopilot
every frame.

```rust
        // Inside loop, before snake.update()

        // AI Control
        snake.autopilot(food.x, food.y);

        // ... then update()
        snake.update(term_cols, term_rows);
```

Task:

Implement set_direction (helper to prevent suicide turns).

Implement autopilot.

Call autopilot in the loop.

Run it! Watch your lambda chase the snowflakes automatically.

Commit: feat: implement basic autopilot AI for screensaver mode

Does it chase the food? (It might be a bit dumb and crash into itself
eventually—that's part of the fun for now!)

## Goal:

Default state: Autopilot (Screensaver mode).

If user presses an Arrow Key: Switch to Manual Mode.

If user presses A (or doesn't touch keys for 5 seconds): Switch back to
Autopilot.

Step 7: Toggle Modes

1. Add a Mode Enum (Optional but clean) or just a bool:

```rust
#[derive(PartialEq)]
enum Mode {
    Auto,
    Manual,
}
```

2. Update main: We need to track the mode.

```rust
    // ... setup ...
    let mut mode = Mode::Auto; // Start in screensaver mode

    while running {
        // 1. Input Handling
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => running = false,

                    // Toggle Mode explicitly
                    KeyCode::Char('a') => mode = Mode::Auto,

                    // Manual Controls -> Switch to Manual Mode automatically
                    KeyCode::Left => { mode = Mode::Manual; snake.set_direction(LEFT); }
                    KeyCode::Right => { mode = Mode::Manual; snake.set_direction(RIGHT); }
                    KeyCode::Up => { mode = Mode::Manual; snake.set_direction(UP); }
                    KeyCode::Down => { mode = Mode::Manual; snake.set_direction(DOWN); }

                    _ => {}
                }
            }
        }

        // 2. Logic
        if mode == Mode::Auto {
            snake.autopilot(food.x, food.y);
        }

        snake.update(term_cols, term_rows);

        // ... drawing ...
    }
```

3. Visual Indicator (Optional): It's helpful to show the user which mode is
   active. Add a text print in the Draw section:

```rust
        // Draw UI
        let mode_text = match mode {
            Mode::Auto => "AUTO (Press Arrows to Play)",
            Mode::Manual => "MANUAL (Press 'a' for Auto)",
        };
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            SetForegroundColor(Color::White),
            Print(mode_text)
        )?;
```

Commit: feat: add manual override and toggle between auto/manual modes

Now you have the best of both worlds! Watch it play, then grab the controls when
it's about to do something stupid.

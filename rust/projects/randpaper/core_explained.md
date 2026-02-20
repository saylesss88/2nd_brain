# core.rs explained

This function is the "engine" of the program. While the CLI handles the user and
the loops handle the timing, this function does the actual work of digging
through your files and picking a winner.

```rs
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Picks a random wallpaper file from `dir`.
///
/// # Errors
///
/// Returns an error if:
/// - the directory cannot be read, or
/// - no supported images (jpg, jpeg, png, bmp) are found in `dir`.
pub fn pick_random_wallpaper<P: AsRef<Path>>(dir: P) -> anyhow::Result<PathBuf> {
    let mut images = Vec::new();
    for entry in WalkDir::new(dir.as_ref()) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let ext = entry.path().extension().unwrap_or_default();
            if ["jpg", "jpeg", "png", "bmp"].contains(&ext.to_str().unwrap_or("")) {
                images.push(entry.path().to_path_buf());
            }
        }
    }
    if images.is_empty() {
        anyhow::bail!("no images found in {:?}", dir.as_ref().display());
    }

    let mut rng = rand::rng();
    let i = rand::Rng::random_range(&mut rng, 0..images.len());
    Ok(images[i].clone())
}
```

1. The Generic Input: `<P AsRef<Path>>`

Instead of just taking a `String`, we use Generics.

- What it means: This function accepts anything that can be looked at as a file
  path.

- Why thats good: You can pass it a `String`, a `&str`, or a `PathBuf` without
  having to manually convert them. This makes it flexible.

2. The `WalkDir` Loop

You are using the walkdir crate, which is much better than the standard
library's read_dir.

- Recursive by default: If you have wallpapers organized in subfolders (e.g.,
  Wallpapers/Nature/ and Wallpapers/Space/), WalkDir will find all of them.

- `let entry = entry?;`: This is where anyhow shines. If the program hits a
  folder it doesn't have permission to read, it will immediately stop and return
  an error back to main.

3. The Extension Filter This block is your "quality control":

```Rust
let ext = entry.path().extension().unwrap_or_default();
if ["jpg", "jpeg", "png", "bmp"].contains(&ext.to_str().unwrap_or("")) {
    images.push(entry.path().to_path_buf());
}
```

- `unwrap_or_default()`: This handles files that have no extension (like a
  LICENSE file) without crashing.

- Case Sensitivity Warning: Note that .contains is case-sensitive here. If you
  have a file named image.PNG (uppercase), this code will actually skip it.

4. anyhow::bail! This is a great macro for "Early Exit." Instead of returning a
   complicated Error object, bail! creates a formatted error message on the fly.
   It's perfect for the logic: "If I found zero images, there is no point in
   continuing; just tell the user why."

5. Random Selection You are using the modern rand syntax:

- `rand::rng()`: Grabs a handle to the thread-local random number generator.

- `random_range(..)`: Picks a number from 0 to the size of your list.

- Indexing: `images[i]` gets the path at that random spot.

A Small Suggestion for Robustness

Since you're on Linux (Wayland), it's very common to have files with uppercase
extensions. You might want to tweak your extension check to be more forgiving:

```Rust
// A slightly more robust check
let ext = entry.path().extension()
    .and_then(|s| s.to_str())
    .unwrap_or("")
    .to_lowercase(); // Convert to lowercase first

if ["jpg", "jpeg", "png", "bmp"].contains(&ext.as_str()) {
    images.push(entry.path().to_path_buf());
}
```

How this connects to your build

Once your hyprland dependencies finish compiling (I hope the -j 1 or the swap
fixed it!), this function will be called by your sway or hyprland modules inside
their .await loops.

Would you like to see how to integrate the time string from your CLI to
determine how often this pick_random_wallpaper function gets called?

---
id: Useful g Keystrokes
aliases:
  - Useful g Keystrokes
tags: []
---

# Useful g Keystrokes

- `gf` - Edit the file located at the filepath under the cursor.
  - You can use `CTRL+W CTRL+F` to open the file in a new window.

- `gx` - Open the file located at the filepath under the cursor in a new tab.

- `gi` - Move to the last insertion you did and switch to INSERT mode.

- `gv` - Start VISUAL mode and use the selection made during the lase VISUAL mode.

- `gn` - Select the match of your last search:
    1. Move to the last searched match.
    2. Switch to VISUAL mode.
    3. select the match.
    4. type `n` to go to the next match.
    5. type `N` to go to the previous match.

- `gI` - Insert text at the beginning of the line, no matter what the first 
  characters are.
   - The keystroke `I` insert text just before the first non-blank characters of
     the line.

-  Try it out pressing `gf` and `gx` on `https://www.google.com/` 
    - Don't forget the trailing slash in your URL.

[[Ranges.md]]

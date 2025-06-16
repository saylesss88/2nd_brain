---
id: Ranges
aliases:
  - Ranges
tags: []
---

# Ranges

- `.` - Represent the current line (often the default range).

- `$` - Represent the last line of the current buffer.

- `%` - Represent the entire file (same as `1,$`).

- `*` - Use the last selection you've made during the last VISUAL mode.

For example, using the command `:d`:

- `:1,40d` - Delete line 1 to 40 included.

- `:2,$d` - Delete every line from the second one till the end of the file.

- `:.,$d` - Delete every line from the current line till the end of the file.

- `:%d` - Delete every line of the file.

## Visual Mode and Range

If you switch to COMMAND-LINE mode after doing some selection in VISUAL mode,
you'll see these two symbols appearing: `<` and `>` with a comma `,` in between.
This is a range too!!

- `<` - Represent the first line you've selected and `>` - the last line.
- Each of these are marks.

- In practice, the ranges '<,'> and * are synonym, but you’ll have more
flexibility with the first. For example, you can execute a command from the
first line you’ve selected till the end of the file with the range '<,$.

[[Vim's Quickfix and Location List.md]]

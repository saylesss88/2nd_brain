---
id: findAndLocate
aliases:
  - commands
  - Find and Locate
tags: []
---

# Find and Locate

Run `find` to find `file` in `dir` as follows:

```bash
find dir -name file -print
```

- The `find` command accepts special pattern-matching characters such as `*`,
  but you must enclose them in single quotes ('\*') to protect the special
  characters from the shell's own globbing feature.

- Rather than searching for a file in real time, `locate` searches an index that
  the system builds periodically. `locate` is much faster than `find`, but if
  you're looking for a newer file, you might want to update your index.

[[head_tail.md]]

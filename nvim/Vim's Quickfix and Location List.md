---
id: Vim's Quickfix and Location List
aliases:
  - Vim's Quickfix and Location List
tags: []
---

# Vim's Quickfix and Location List

## Quickfix Lists

You can think of a quickfix list as a set of positions in one or multiple files.

**BASICS**

If you run `:vimgrep hello *`:
   1. It will search the pattern "hello" in every file of your working directory.
   2. It will populate a quickfix list with every position matching the pattern.
   3. It will move your cursor to the first position in the quickfix list. 

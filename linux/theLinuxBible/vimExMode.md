---
id: vimExMode
aliases:
  - vimExMode
tags: []
---

# vimExMode

The following are examples of some `ex` commands for searching for and changing
text.

`:g/Local` : Searches for the word `Local` and prints every occurrence of that
line from the file.

`:s/Local/Remote` : Substitutes the first occurrence of `Local` with `Remote`.

`:g/Local/s//Remote` : Substitutes the first occurrence of the word `Local` on
every line of the file with the word `Remote`.

`:g/Local/s//Remote/g` : Substitutes every occurrence of the word `Local` with
the word `Remote` on every line of the file.

`:g/Local/s//Remote/gp` : Substitutes every occurrence of the word `Local` with
the word `Remote` on every line of the file and prints the result.

[[findingFiles.md]]

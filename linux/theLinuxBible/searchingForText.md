---
id: searchingForText
aliases:
  - Searching for text
tags: []
---

# Searching for text

To search for the next or previous occurrence of text in the file, use either
the `/` or `?` keys:

`/hello`: Searches forward for the next occurrence of `hello` in the file.

`?goodbye`: Searches backward for the word `goodbye`.

`/The.*foot`: Searches forward for a line that has the word `The` in it and
also, after that at some point, the word `foot`.

`?[pP]rint`: Searches backward for a line that has the word `print` or `Print`.

[[vimExMode.md]]

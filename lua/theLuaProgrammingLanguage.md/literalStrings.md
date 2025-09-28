---
id: literalStrings
aliases:
  - literalStrings
tags:
  - lua
---

# literalStrings

We can delimit literal strings by single or double matching quotes:

```lua
a = "a line"
b = "another line"
```

- They are equivalent; the only difference is that inside each kind of quote we
can use the other quote without escapes.

```lua
> print("one line\nnext line\n\"in quotes\", 'in quotes'")
one line
next line
"in quotes", 'in quotes'
> print('a backslash inside quotes: \'\\\'')
a backslash inside quotes: '\'
> print("a simpler way: '\\'")
a simpler way: '\'
```



---
title: Quoting
date: 2024-10-11
tags: [tag1, tag2]
---

# Quoting

```bash
echo this is a      test
this is a test
echo The total is $100.00
The total is 00.00
```

In the first example, _word splitting_ by the shell removed extra whitespace
from the echo command's list of arguments.

In the second example, parameter expansion substituted an empty string for the
value of $1 because it was an undefined variable.

The shell provides a mechanism called quoting
to selectively suppress unwanted expansions.

## Double Quotes

If you place text inside double quotes, all the special characters used by the
shell lose their special meaning and are treated as ordinary characters.

- The exceptions are the $ (dollar sign), \ (backslash), and ` (back tick).

- This means that word splitting, pathname expansion, tilde expansion, and
  brace expansion are suppressed, but parameter expansion, arithmetic expansion,
  and command substitution are still carried out.

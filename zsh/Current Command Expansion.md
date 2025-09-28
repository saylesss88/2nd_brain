---
id: Current Command Expansion
aliases:
  - Current Command Expansion
tags: []
---

# Current Command Expansion

You can also expand arguments of the current command line, making it easier to
reuse parts of commands.

- `!#^` - Expand the first argument of the current command.

- `!#$` - Expand the last argument of the current command.

> Note: This is useful for copying or renaming a file with a long name.

- `cp reallylongnamefilebackup-!#$` - Would expand to file name after hitting 
  tab.

## Quick Fixes

- `fc` - Fix command: opens the last command in the editor.

- `r` - Repeat the last command (can be combined with substitution, e.g.
  `r foo=bar`).

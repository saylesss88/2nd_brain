---
title: The Environment
date: 2024-10-13
tags: [tag1, tag2]
---

# The Environment

The shell stores tow basic types of data in the environment, _environment
variables_ and _shell variables_.

- _Shell variables_ are bits of data placed there by bash, and _environment
  variables_ are basically everything else.

## Examining the Environment

To see what's stored in the environment, you can either use the `set` built in
or the `printenv` command.

```bash
printenv | less
```

- This prints out all the environment variables and their values.

To find a specific variable:

```bash
printenv USER
```

The set command, when used without options or arguments, will dis-
play both the shell and environment variables, as well as any defined shell
functions.

```bash
set | less
```

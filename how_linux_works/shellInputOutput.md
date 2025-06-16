---
id: shellInputOutput
aliases:
  - Shell Input and Output
tags: []
---

# Shell Input and Output

To send the output of `command` to a file instead of the terminal, use the `>`
redirection character:

`command > file`

Or append the output to the file instead of overwriting it:

`command >> file`

To send the standard output of a command to the standard input of another
command, use the pipe character (`|`):

```bash
head /proc/cpuinfo
head /proc/cpuinfo | tr a-z A-Z
```

- You can send output through as many piped commands as you wish;
  just add another pipe before each additional command.

## Standard Error

Occasionally, you may redirect standard output but find that the program still
prints something to the terminal. This is called _standard error_(stderr); it's
an additional output stream for diagnostics and debugging.

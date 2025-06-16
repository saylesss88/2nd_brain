---
id: Pipelines
aliases: []
tags: []
---

## Pipelines

The ability of commands to read data from standard input and send to
standard output is utilized by a shell feature called **pipelines**.

- Using the pipe operator | (vertical bar), the standard output of one command can be piped into the standard input of another.

   command1 | command2

- Remember how `less` accepts standard input?

```bash
ls -l /usr/bin | less
```

- This is extremely handy! Using this technique, we can conveniently examine the
  output of any command that produces output on standard output.

[[Filters.md]]

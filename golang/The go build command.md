---
id: The go build command
aliases:
  - The go build command
tags: []
---

# The go build command

The go run command is useful for quickly checking whether the program works, but
if we actually want to distribute our software to customers, a more convenient
way to do that is to build an executable file. Let’s see how that works.

  To create an executable, run the command:

```bash
go build -o add ./cmd/calculator
```

  If there are no errors, this command won't produce any output, but you'll find
that a new file has been created in the current directory, named `add`.

To run it, use the program name itself as the command:

```bash
./add
```

4

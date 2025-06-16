---
id: pipes
aliases:
  - Pipes
tags:
  - bash
  - commands
---

# Pipes

Instead of redirecting the output of a command to a file, you can redirect the
output of one command to the input of another command.(Piping)

- Don't think of piping as running two commands back to back. The Linux system
  actually runs both commands at the same time, linking them together internally
  in the system. As the first command produces output, it is sent immediately to
  the second command.

- There's no limit to the number of pipes you can use in a command.

- One of the most popular uses of piping is piping the results of commands that
  produce long output to the `more` or `less` command.

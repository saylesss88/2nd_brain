---
id: Command History and Expansion
aliases:
  - Command History and Expansion
tags: []
---

# Command History and Expansion

- `Ctrl + r` - Display command history and navigate history buffer with  
  `Ctrl + n` and `Ctrl + p`(provided no vim key mode is enabled)

- `Alt + .` or `<Esc> + .` - Insert the last argument of the previous command
  (same as `!$`)

- `Esc + _` - Insert the last word of the previous command, cycles through
  arguments.

- `!!` - Repeat last command, useful for `sudo !!` to append sudo to previous command

- `!:0` - Repeat the previous command with no arguments

- `!vi` - Run the most recent command that started with 'vi'.

## History Modification

Modifying previously executed commands can help you correct mistakes or adjust
commands for different tasks without starting from scratch.

- `!!:s/x/y` - Substituts 'x' with 'y' in the previous command.

- `^foo^bar` - Replaces 'foo' with 'bar' in the previous command.

> These history modification shortcuts are useful for fixing typos or changing
parameters in commands you've just executed.

[[Current Command Expansion.md]]

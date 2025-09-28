---
id: shellBuiltins
aliases:
  - shellBuiltins
tags: []
---

# shellBuiltins

Built-in commands are contained within the shell itself. When the name of a
built-in command is used as the first word of a simple command, the shell
executes the command directly, without creating a new process.

Built-in commands are necessary to implement functionality impossible or
inconvenient to obtain with separate utilities.

Bash supports 3 types of built-in commands:

- Bourne Shell built-ins:
  :, ., break, cd, continue, eval, exec, exit, export, getopts, hash, pwd, readonly, return, set, shift,
  test, [, times, trap, umask and unset.
  • Bash built-in commands:
  alias, bind, builtin, command, declare, echo, enable, help, let, local, logout, printf, read, shopt,
  type, typeset, ulimit and unalias.
  • Special built-in commands:
  When Bash is executing in POSIX mode, the special built-ins differ from other built-in commands in
  three respects:

1. Special built-ins are found before shell functions during command lookup.
2. If a special built-in returns an error status, a non-interactive shell exits.
3. Assignment statements preceding the command stay in effect in the shell environment after
   the command completes.
   The POSIX special built-ins are :, ., break, continue, eval, exec, exit, export, readonly, return, set,
   shift, trap and unset.
   Most of these built-ins will be discussed in the next chapters. For those commands for which this is not the
   case, we refer to the Info pages.

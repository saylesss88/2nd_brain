---
id: Zsh Line Editor(ZLE)
aliases:
  - Zsh Line Editor(ZLE)
tags: []
---

# Zsh Line Editor(ZLE)

If the ZLE option is set which is the default and the shell input is attached to
the terminal, the user is able to edit command lines.

There are two display modes:

  1. Multiline mode, is the default. It only works if the TERM parameter is set
     to a valid terminal type that can move the cursor up.

  2. Single line mode, is used if TERM is invalid or incapable of moving the
     cursor up, or if the SINGLE_LINE_ZLE parameter is set.

In vi-mode by typing a number before entering a command, generally the next
command entered will be repeated that many times.

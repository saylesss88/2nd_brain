---
id: theShell
aliases:
  - The Shell
tags: []
---

# The Shell

**About Shells and Terminal Windows**

- There are several ways to get to a shell interface in Linux. Three of the most
  common are:

    - The shell prompt
    - The Terminal window
    - The Virtual Console

## Using the Shell Prompt

If your Linux system has no graphical user interface (or one that's not working
at the moment), you will most likely see a shell prompt after you log in.

- The default prompt for a regular user is simply a dollar sign:

    - `$`

- The default prompt for root is the hash tag:

    - `#`

- In most Linux systems, the `$` and `#` prompts are preceded by your username,
  system name, and current directory name. For a user named `jr` on a computer
  named `xps` in the directory `/usr/share/`:

    - `[jr@xps share]$`
  
- In the examples, the dollar sign is meant to be run without extended
  privaleges.

- Prompts that start with `#` are meant to be run as root.

### Using a Terminal Window

  Wih a desktop GUI running, you can open a Terminal emulator program (Terminal
  Window) to start a shell.

#### Using Virtual consoles

Most distros that include a desktop interface start multiple virtual consoles
running on the computer.

- `Virtual consoles` are a way to have multiple shell sessions open at once in
  addition to the desktop GUI.

    - You can switch between virtual consoles by holding the CTRL and ALT keys
      and pressing a function key between F1 and F6 on most systems.

    - The GUI is typically located on one of the first two virtual consoles, and
      the other six virtual consoles are typically text-based virtual consoles.

    - You can return to the GUI (if one is running) by holding the CTRL+ALT+F1.

    - Newer systems, now start the gdm (the login screen) persistently on tty1
      to allow multiple simultanious GUI sessions: the gdm is on tty1, the first
      desktop is started on tty2, and so on.

[[understandingCommandSyntax.md]]

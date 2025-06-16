---
id: User Space
aliases:
  - User Space
tags: []
---

# User Space

The main memory that the kernel allocates for user processes is called *user
space*. 

  - Because a process is simply a state (or image) in memory, user space also
    refers to the memory for the entire collection of running processes.

  * Most of the real action on a Linux system happens in user space. All
    processes are essentially equal from the kernel's point of view, they
    perform different tasks for users.

  - There's a rudimentary service level (or layer) structure to the kinds of
    system components that user processes represent.

  - Basic services are at the bottom level (closest to the kernel), utility
    services are in the middle, and applications that users touch are at the
    top.

[[Users.md]]

---
id: Levels and Layers of Abstraction
aliases:
  - Levels and Layers of Abstraction
tags: []
---

# Levels and Layers of Abstraction

Using abstraction to split computing systems into components makes things easier
to understand, but it doesn't work without organization.

- We arrange components into *layers* or *levels*, classifications (or groupings)
of components according to where the components sit between the user and the
hardware.

    - Web browsers, games, and such sit at the top layer; at the bottom layer we
      have the memory in the computer hardware -- the 0s and 1s.

    - The operating system occupies many of the layers in between.

- A linux system has three main levels:

    - The *hardware* is at the base. Hardware includes the memory, CPUs to
      perform computation and to read from and write to the memory.

    - Devices, such as disks and network interfaces are also part of hardware.

The next level up is the *kernel*, which is the core of the operating system.

  - The kernel is software residing in memory that tells the CPU where to look
     for its next task.

  - Acting as the mediator, the kernel manages the hardware (especially main
        memory) and is the primary interface between the hardware and any
        running program.

  - *Processes* -- the running programs that the kernel manages -- collectively
    make up the system's upper level, called *user space*.
> [!Note] A more specific term for process is *user process*, regardless of
> whether a user directly interacts with the process. All web servers run as
> user processes.

There is a critical difference between how the kernel and the user processes
run: the kernel runs in *kernel mode*, and the user processes run in *user mode*

    - Code running in kernel mode has unrestricted access to the processor and
      main memory.

    - This is a powerful but dangerous privilege that allows the kernel to
      easily corrupt and crash the entire system.

    - The memory area that only the kernel can access is called the *kernel
      space*.

User mode, restricts access to (usually quite small) subset of memory and safe
CPU operations.

    - User space refers to the parts of main memory that the user processes can
      access.

    - If a process makes a mistake and crashes, the consequences are limited and
    can be cleaned up by the kernel.

    - This means that if your web browser crashes, it probably won't take down
      the scientific computation that has been running in the background for
      days.

[[Hardware Understanding Main Memory.md]]

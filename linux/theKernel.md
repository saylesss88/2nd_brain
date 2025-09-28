---
id: The Kernel
aliases:
  - The Kernel
tags: []
---

# The Kernel

Nearly everything the kernel does revolves around main memory.

  - One of the kernel's tasks is to split memory into many subdivisions, and it
    must maintain certain state information about those subdivisions at all
    times.

  - Each process gets its own share of memory, and the kernel must ensure that
    each process keeps to its share.


The kernel is in charge of managing tasks in four general system areas:

  - **Processes** The kernel is responsible for determining which processes are
    allowed to use the CPU.

  - **Memory** The kernel needs to keep track of all memory -- what is currently
    allocated to a particular process, what might be shared between processes,
    and what is free.

  - **Device drivers** The kernel acts as an interface between hardware (such as
    a disk) and processes. It's usually the kernels job to operate hardware.

  - **System calls and support** Processes normally use system calls to
    communicate with the kernel.

[[Process Management.md]]

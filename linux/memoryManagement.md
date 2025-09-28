---
id: Memory Management
aliases:
  - Memory Management
tags: []
---

# Memory Management

The kernel must manage memory during a context switch, which can be a complex
job. The following conditions must hold:

- The kernel must have its own private area in memory that user processes can't
  access.

- Each user process needs its own section of memory.

- One user process may not access the private memory of another user process.

- User processes can share memory.

- Some memory in user processes can be read-only.

- The system can use more memory than is physically present bu using disk space
  as auxiliary.

Modern CPUs include a *memory management unit* (MMU) that enables a memory
access scheme called *virtual memory* (VM).

  - When using virtual memory, a process does not directly access the memory by
    its physical location in the hardware. Instead, the kernel sets up each
    process to act as if it had an entire machine to itself.

  - When the precess accesses some of its memory, the MMU intercepts the access
    and uses a memory address map to translate the memory location from the
    process point of view into an actual physical memory location in the
    machine.

> [!NOTE] The implementation of a memory address map is called a page table.

[[Device Drivers and Management.md]]

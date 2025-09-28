---
id: Process Management
aliases:
  - Process Management
tags: []
---

# Process Management

*Process management* describes the starting, pausing, resuming, scheduling, and
terminating of processes.

  - Starting and stopping processes is pretty straightforward, but describing
    hos a process uses the CPU in its normal course of operation is a bit more
    complex.

  - On any modern O.S., many processes run simultaneously.

  - You may have a web browser open and a spreadsheet open at the same time,
    however things are not as they appear. Processes behind these applications
    typically don't run at *exactly* the same time.

With a one core system, many processes may be *able* to use the CPU, but only
one process can actually use the CPU at any given time.

  - In practice, each process uses the CPU for a small fraction of a second,
    then pauses; then another process uses the CPU for another fraction of a
    second, then another process takes a turn, and so on.

  - The act of a process giving up control of the CPU to another process is
    called a *context switch*.

  - Each piece of time -- called a *time slice* -- gives a process enough time
    for significant computation (and a process often finishes its current task
      in a single slice).

    - However, because the slices are so small, humans can't perceive them, and
      the system appears to be running multiple processes at the same time (a
      capability known as *multitasking*).

The kernel is responsible for context switching. To understand how this
works, let’s think about a situation in which a process is running in user
mode but its time slice is up. Here’s what happens:

1. The CPU (the actual hardware) interrupts the current process based on an
   internal timer, switches to kernel mode, and hands control back to the
   kernel.

2. The kernel records the current state of the CPU and memory, which will be
   essential to resuming the process that was just interrupted.

3. The kernel performs any tasks that might have come up during the preceding
   time slice (such as collecting data from input and output, or I/O, operations)

4. The kernel is now ready to let another process run. The kernel analyzes the
   list of processes that are ready to run and chooses one.

5. The kernel prepares the memory for this new process and then prepares the
   CPU.

6. The kernel tells the CPU how long each time slice for the new process should
   last.

7. The kernel switches the CPU into user mode and hands control of the CPU to
   the process.

  The context switch answers the important question of *when* the kernel runs.
  The answer is that it runs *between* process time slices during a context
  switch.

[[Memory Management.md]]

---
id: Hardware Understanding Main Memory
aliases:
  - Hardware Understanding Main Memory
tags: []
---

# Hardware Understanding Main Memory

Of all the hardware in a computer, the main memory is the most important.

- Each slot for a 0 or 1 is called a *bit*.

- This is where the running kernel and processes reside -- they're just a big
  collection of bits.

- All input and output from peripheral devices flows through the main memory,
  also as a bunch of bits.

- A CPU is just an operator on memeory; it reads its instructions and data from
  the memory and writes data back out to the memory.

    - You'll often hear the term *state* in reference to memory, processes, the
      kernel, and other parts of a computer system.

    - Strictly speaking, a state is a particular arrangement of bits.

    - For example, if you have four bits in your memory, 0010, 0001, and 1011
      represent three different states.

    - Instead of describing a state using bits, you describe what something
has done or is doing at the moment. For example, you might say, “The pro-
cess is waiting for input” or, “The process is performing Stage 2 of its startup.”

> [!Note] Because it's common to refer to the state in abstract terms rather
> than to the actual bits, the term image refers to a particular physical
> arrangement of bits.

[[The Kernel.md]]

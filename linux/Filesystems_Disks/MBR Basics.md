---
id: MBR Basics
aliases:
  - MBR Basics
tags: []
---

# MBR Basics

The MBR table in the example contains primary, extended, and logical partitions.

A *primary partition* is a normal subdivision of the disk; partition `1` in the
example.

- The basic MBR has a limit of four primary partitions, so if you want more than
  four, you must designate one as an extended partition.
    - An extended partition breaks down into *logical partitions*, which the
      O.S. can then use.

[[LVM Partitions.md]]

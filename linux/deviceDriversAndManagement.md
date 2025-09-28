---
id: Device Drivers and Management
aliases:
  - Device Drivers and Management
tags: []
---

# Device Drivers and Management

The kernel's role with devices is relatively simple. A device is typically
accessible only in kernel mode because imporper access (such as a user process
asking to turn off the power) could crash the machine.

A notable difficulty is that different devices rarely have the same programming
interface, even if the devices perform the same task (for example, two different
network cards). 

  - Therefore, device drivers have traditionally been part of the kernel and
    they strive to present a uniform interface to user processes in order to
    simplify the software developer's job.

[[System Calls and Support.md]]

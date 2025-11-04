## Cgroups

Control groups (cgroups) are a Linux kernel mechanism for fine-grained control
of resources.

Cgroups are a facility built into the kernel that allow the admin to set
resource utilization limits on any process on the system.

In general cgroups control:

- The number of CPU shares per process

- The limits on memory per process.

- Block Device I/O per process.

- Which network packets are identified as the same type so that another app can
  enforce traffic rules.

## Why cgroups are important

1. Resource limiting: Keeping certain acceptable boundaries for CPU, RAM, block
   device I/O, and device groups.

> NOTE: The device groups CGroup can be a key component in your system's
> comprehensive security strategy. Device groups include controlling permissions
> for read, write, and `mknod` operations.

- `mknod` was initially designed to populate all things that show up in `/dev/`
  These are things like hard drives, USB interfaces, or other devices that might
  exist on a system.
  - Most modern Linux systems use `udev` to automatically populate this virtual
    filesystem with things detected by the kernel. `mknod` also allows multiple
    programs to communicate with each other by creating a named pipe. The main
    thing to grasp is that this facilitates passing information from one program
    to another.

2. Prioritization

3. Accounting

4. Process control

It enhances your security posture quite a bit. While a typical lynux
installation uses cgroups by default, it doesn't put any restrictions upon
processes. You can impose restrictions by default if you so choose. You can also
restrict access to specific devices for specific users, groups, or processes,
which helps to further lock down your system.

You can also do performance tuning with cgroups.

## How cgroups work

Cgroups are a mechanism for controlling certain subsystems in the kernel. These
subsystems, such as devices, CPU, RAM, network access, and so on, are called
_controllers_ in cgroup terminology.

- [RedHat cgroups](https://www.redhat.com/en/blog/cgroups-part-one)

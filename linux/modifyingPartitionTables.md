---
id: Modifying Partition Tables
aliases:
  - Modifying Partition Tables
tags: []
---

# Modifying Partition Tables

- `parted` Modifies partitions as you issue commands. (text-based)

- `gparted` GUI

- `fdisk` You design the partition table before making the actual changes to the
  disk, and it makes the changes only when you exit the program.

  There are a few ways to see the partition changes:

    - Use `udevadm` to watch the kernel event changes. `udevadm monitor --kernel` 
    will show the old partition devices being removed and the new ones added.

    - Check `/proc/partitions` for full partition information.

    - Check `/sys/block/device/` for altered partition system interfaces or
      `/dev` for altered partition devices.

> [!note] If you absolutely must confirm your modifications to a partition
> table, you can use the `blockdev` command to perform the old-style system call
> that `fdisk` issues. For example, to force the kernel to reload the partition
> table on `/dev/sdf`, run this:

```bash
sudo blockdev --rereadpt /dev/sdf
```
[[Creating a Partition Table]]

---
id: Creating a Partition Table
aliases:
  - Creating a Partition Table
tags: []
---

# Creating a Partition Table

This example shows the following scenario:

- 4GB disk (a small USB flash device, unused; if you want to follow this
example, use any size device that you have at hand)

- MBR-style partition table
Two partitions intended to be populated with an ext4 filesystem:
200MB and 3.8GB

- Disk device at /dev/sdd; you’ll need to find your own device location
with lsblk

```bash
sudo fdisk /dev/sdd
```

You'll get an introductory message like this:

```bash
Command (m for help):
```

  - First, print the current table with the `p` command, Your interaction will
    look something like this:

```bash
Command (m for help): p
Disk /dev/sdd: 4 GiB, 4284481536 bytes, 8368128 sectors
Units: sectors of 1 * 512 = 512 bytes
Sector size (logical/physical): 512 bytes / 512 bytes
I/O size (minimum/optimal): 512 bytes / 512 bytes
Disklabel type: dos
Disk identifier: 0x88f290cc
Device
/dev/sdd1
Boot Start
End Sectors Size Id Type
2048 8368127 8366080
4G c W95 FAT32 (LBA)
```
Use `d` to delete a partition. Use `n` to create a new partition. Use `w` to
write the partition table to disk.

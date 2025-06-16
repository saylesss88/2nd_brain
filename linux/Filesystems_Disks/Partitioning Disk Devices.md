---
id: Partitioning Disk Devices
aliases:
  - Partitioning Disk Devices
tags: []
---

# Partitioning Disk Devices

## Viewing a Partition Table

```bash
sudo parted -l 
Model: THNSN5512GPUK NVMe TOSHIBA 512GB (nvme)
Disk /dev/nvme0n1: 512GB
Sector size (logical/physical): 512B/512B
Partition Table: gpt
Disk Flags:

Number  Start   End     Size    File system  Name                  Flags
 1      1049kB  630MB   629MB   fat32        EFI System Partition  boot, esp
 2      630MB   1704MB  1074MB  ext4                               bls_boot
 3      1704MB  512GB   510GB   btrfs


Model: Unknown (unknown)
Disk /dev/zram0: 8590MB
Sector size (logical/physical): 4096B/4096B
Partition Table: loop
Disk Flags:

Number  Start  End     Size    File system     Flags
 1      0.00B  8590MB  8590MB  linux-swap(v1)
```
:> [!WARNING]
> Watch out for the unit sizes when reading partition tables. `parted` shows
> approximated size based on what it thinks is easiest to read. `fdisk -l` shows
> an exact number r, but in most cases, the units are 512-byte “sectors,” which
> can be confusing because it might look like you’ve doubled the
> actual sizes of your disk and partitions. A close look at the fdisk partition
> table view also reveals the sector size information.

[[MBR Basics.md]]

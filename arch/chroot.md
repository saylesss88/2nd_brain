---
title: chroot
date: 2024-09-24
tags: [tag1, tag2]
---

# chroot

A `chroot` is an operation that changes the apparent root directory for the
current running process and their children.

## Prepare new root location

The chroot target should be a directory which contains a file system hierarchy.

- In the installation guide, this directory would be /mnt. For an existing
  installation, you need to mount existing partitions into /mnt yourself:

Run lsblk and note the partition layout of your installation. It will be usually
something like /dev/sdXY or if you have an NVMe drive /dev/nvme0nXpY.

Mount the file system:

```bash
# mount /dev/sdXY /mnt
```

If you have an EFI system partition and need to make changes in it:

```bash
# mount /dev/sdXZ /mnt/esp
```

### Using arch-chroot

The bash script `arch-chroot` is part of the `arch-install-scripts` package.

- arch-chroot wraps the chroot(1) command while ensuring that important
  functionality is available, e.g. mounting /dev, /proc and other API filesystems,
  or exposing /etc/resolv.conf to the chroot.

**Enter a chroot**

```bash
# arch-chroot /path/to/new/root
```

- You can now do most of the operations available from your existing installation. Some tasks which needs D-Bus will not work as noted in `#Usage`.

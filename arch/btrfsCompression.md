---
title: How to enable compression for btrfs
date: 2024-10-09
tags: [tag1, tag2]
---

# Enable compression for btrfs

Typically the compression can be enabled on the whole filesystem, specifed for
the mount point. Note that the compression mount options are shared among all
mounts of the same filesystem, either bind mounts or subvolume mounts.

```bash
mount -o compress=zstd /dev/mmcblk0p2 /mnt
```

- This will enable `zstd` compression on the default level (3). This level can
  be specified manually too like `zstd:3`.

Defragment the whole file and start compression, regardless of the mount option:

```bash
btrfs filesystem defrag -czstd file
```

- The compression algorithm is not persistent and applies only to the
  defragmentation command.

Persistent settings on a per-file basis can be set in two ways:

```bash
chattr +c file
btrfs property set file compression zstd
```

The first command is using legacy interface of file attributes inherited from
ext2 filesystem and is not flexible, so by default the zlib compression is set.
The second command sets a property on the file with the given algorithm.

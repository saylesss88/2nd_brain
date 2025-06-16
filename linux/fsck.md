---
title: fsck (file system check)
date: 2024-10-31
tags: [tag1, tag2]
---

# fsck (file system check)

`fsck` stands for "file system check" and is used to check and optionally repair
one or more Linux file systems.

- The Arch Linux boot process conveniently takes care of the fsck procedure for
  you and will check all relevant partitions on your drive automatically on
  every boot. Hence, there is usually no need to resort to the command-line.

## Boot time checking

There are two players involved:

1. `mkinitcpio` offers you the option to fsck your root file system before
   mounting it via the `fsck` hook. If you do this you should mount root
   read-write via the appropriate `rw` kernel parameter.

2. `systemd` will fsck all file systems having a fsck pass number greater than 0
   For the root file system, it also has to be mounted read-only initially
   with the `ro` kernel parameter and only then remounted read-write from
   `fstab`.(the `defaults` mount option implies `rw`)

- You can run the `fsck` command on unmounted file systems only. For most
  filesystems, you can just unmount the filesystem to check it and then remount it
  when you’re finished. However, because the root filesystem contains all the
  core Linux commands and log files, you can’t unmount it on a running system.
  This is a time where having a Linux LiveCD comes in handy! Just boot your
  system with the LiveCD, and then run the fsck command on the root filesystem!

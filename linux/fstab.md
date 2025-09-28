---
title: fstab
date: 2024-09-22
---

# fstab

Your Linux system's filesystem table, aka `fstab`, is a configuration table
designed to ease the burden of mounting and unmounting file systems to a
machine.

- The `fstab` file is designed to configure a rule where specific file systems
  are detected, then automatically mounted in the user's desired order every
  time the system boots.

## Usage

A simple `/etc/fstab`, using file system UUIDs:

```bash
/etc/fstab

# <device>                                <dir> <type> <options> <dump> <fsck>
UUID=0a3407de-014b-458b-b5c1-848e92a327a3 /     ext4   defaults  0      1
UUID=f9fe0b69-a280-415d-a03a-a32752370dee none  swap   defaults  0      0
UUID=b411dc99-f0a0-4c87-9e05-184977be8539 /home ext4   defaults  0      2
```

- `<device>` describes the block special device or remote file system to be
  mounted.

- `<dir>` is the directory where the file system is mounted.

- `<type>` is the file system type.

- `<options>` the associated mount point options

- `<dump>` is checked by the dump utility. This field is usually set to `0`,
  which disables the check.

- `<fsck>` sets the order for file system checks at boot time. For the root
  device it should be `1`, for other devices it should be `2` or `0` to disable
  checking.

> [!TIP] The `auto` type lets the mount command guess what type of file system
> is used If the root file system is `btrfs` or `XFS`, the fsck should be set to
> `0` instead of `1`.

- All specified devices within /etc/fstab will be automatically mounted on
  startup and when the -a flag is used with mount(8) unless the noauto option is
  specified. Devices that are listed and not present will result in an error
  unless the `nofail` option is used.

## Identifying file systems



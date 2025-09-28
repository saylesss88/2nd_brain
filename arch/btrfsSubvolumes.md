---
title: Subvolumes
date: 2024-10-09
tags: [tag1, tag2]
---

# Subvolumes

a BTRFS subvolume is a part of a filesystem with its own independent
file/directory hierarchy and inode number namespace. Subvolumes can share file
extents.

- A snapshot is also a subvolume, but with a given initial content of the
  original subvolume. A subvolume has always inode number 256.

A subvolume looks like a normal directory, with some additional operations
described below.

- Subvolumes can be renamed or moved, nesting subvolumes is not restricted but
  has some implications regarding snapshots.

- The numeric id (called subvolid or rootid) of the subvolume is persistent and
  cannot be changed.

A subvolume in BTRFS can be accessed in two ways:

- like any other directory that is accessible to the user

- like a separately mounted filesystem (options subvol or subvolid)

In the latter case the parent directory is not visible and accessible. This is
similar to a bind mount, and in fact the subvolume mount does exactly that.

A freshly created filesystem is also a subvolume, called _top-level_, internally
has an id 5. This subvolume cannot be removed or replaced by another subvolume.
This is also the subvolume that will be mounted by default, unless the default
subvolume has been changed.

- A snapshot is a subvolume like any other, with given initial content. By
  default, snapshots are created read-write. File modifications in a snapshot do
  not affect the files in the original subvolume.

- Subvolumes can be given capacity limits, through the qgroups/quota facility,
  but otherwise share the single storage pool of the whole btrfs filesystem. They
  may even share data between themselves (through deduplication or snapshotting).

> [!NOTE] A snapshot is not a backup: snapshots work by use of BTRFS'
> copy-on-write behavior. A snapshot and the original it was taken from
> initially share all of the same data blocks. If that data is damaged in some
> way, then the original and the snapshot will both be damaged.

---
id: rsync
aliases: []
tags: []
---

`rsync` or remote synchronization is a software utility for Unix-Like systems
that efficiently sync files and directories between two hosts or machines.

  One is the source or the local-host from which the files will be synced,
  
  The other is the remote-host, on which synchronization will take place.

There are basically two ways in which rsync can copy/sync data:

  - Copying/syncing to/from another host over any remote shell like ssh, rsh.

  - Copying/syncing through rsync daemon using TCP.

Rsync is famous for its delta-transfer algorithm, in which it copies only the
differences between the source files present in the local-host and the existing
files in the destination or the remote-host.

Example:

```bash
rsync local-file user@remote-host:remote-file
```

**What happens Here:**
  Rsync will first use SSH to connect as the user to remote-host and will ask
for user's password Once connected, it will invoke the remote host’s rsync,
and then the two programs will determine what parts of the local-file need
to be copied so that the remote file matches the local one. Please note the
following behavior of rsync:.

  - Files that do not exist on the remote-host are copied.

  - Files that have been updated will be synced, rsync will copy only the
  changed parts of files to the remote host.

  - Files that are exactly the same will not be copied.

**Syntax of `rsync` command in Linux:**

```bash
rsync [options] source [destination]
```



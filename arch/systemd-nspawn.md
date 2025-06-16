---
title: systemd-nspawn
date: 2024-09-24
tags: [arch, tag2]
---

# systemd-nspawn

`systemd-nspawn` is like the `chroot` command, but it is a chroot on steroids.

- systemd-nspawn may be used to run a command or operating system in a
  light-weight namespace container. It is more powerful than chroot since it fully
  virtualizes the file system hierarchy, as well as the process tree, the various
  IPC subsystems and the host and domain name.

## Installation

`systemd-nspawn` is part of and packaged with `systemd`

[[chroot.md]]

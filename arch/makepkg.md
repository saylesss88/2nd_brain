---
title: makepkg
date: 2024-09-24
tags: [arch, tag2]
---

## makepkg

`makepkg` is a script to automate the building of packages. The requirements for
using the script are a build-capable Unix platform and a `PKGBUILD`

- `makepkg` is provided by the pacman package.

## Configuration

The system configuration is available in `/etc/makepkg.conf`, but user-specific
changes can be made in `$XDG_CONFIG_HOME/pacman/makepkg.conf` or
`~/.makepkg.conf`.

- Also, system wide changes can be made with a drop-in file
  `/etc/makepkg.conf.d/makepkg.conf`. It is recommended to review the
  configuration prior to building packages.

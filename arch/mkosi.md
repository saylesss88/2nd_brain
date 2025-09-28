---
title: mkosi
date: 2024-09-24
tags: [arch, tag2]
---

# What is `mkosi`

`mkosi` was originally written as a tool to simplify hacking on systemd and for
experimenting with images using many of the new concepts being introduced in
systemd at the time. In the meantime, it has evolved into a general purpose
image builder that can be used in a multitude of scenarios.

## Installation

```bash
pipx install git+https://github.com/systemd/mkosi.git
mkosi --version`
sudo pacman -S bubblewrap systemd-nspawn
```

[[systemd-nspawn.md]]

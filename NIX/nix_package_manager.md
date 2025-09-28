---
title: NIX package manager
date: 2024-12-01
tags: [tag1, tag2]
---

# NIX package manager

## Declarative Configuration

The only configuration file associated with Nix is `nix.conf` (usually found in
`/etc/nix/nix.conf`), which is used to configure the Nix package manager.

Since most scenarios involve using Nixpkgs, configuration of
`~/.config/nixpkgs/config.nix` is often also helpful; here package overrides can
be specified.

`Home Manager` is the preferred way to manage declarative environments for a
single user.

**Imperative Operations**

To a large extent, package and environment management in Nix is imperative; user environments - including package installation and removal - is managed with the nix-env command, while nix-channels determine which version of Nixpkgs is used, and thus which version your packages will have.

Common `nix-env` commands:
| Description | Command |
|-------------------------|--------------------------------|
| Searching for packages | nix search nixpkgs packagename |
| Install a package | nix-env -iA packagename |
| List installed packages | nix-env -q |
| Uninstall a package | nix-env -e packagename |
| Upgrade packages | nix-env -u |

---
id: nix_profiles
aliases: []
tags:
  - profiles
---

# Nix Profiles

Profiles and user environments are Nix's mechanism for implementing the ability
to allow different users to have different configurations, and to do atomic
upgrades and rollbacks.

In Nix, packages are stored in unique locations in the Nix store `/nix/store/`.
So if two packages differ in any way, they end up in different locations in the
file system, so they don't interfere with each other.

Rather than type the long hashes Nix sets up the PATH environment variable to
include the `bin` directory of every package we want to use, but this isn't very
convenient because changing PATH doesn't take effect for already existing
processes.

The solution Nix uses is to create directory trees of symlinks to activated
packages. These are called _user environments_ and they are packages themselves
(though generated automatically by `nix-env`), so they too reside in the Nix
store.

[[nix-env.md]]

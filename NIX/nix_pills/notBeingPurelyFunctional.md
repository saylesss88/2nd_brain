---
title: Not Being Purely Functional
date: 2024-11-30
tags: [tag1, tag2]
---

# Not Being Purely Functional

Most widely used package managers (dpkg, rpm, etc) mutate the global state of
the system.

- If a package `foo-1.0` installs a program to `/usr/bin/foo`, you cannot
  install `foo-1.1` as well, unless you change the installation paths or the
  binary name. But changing the binary names means breaking users of the binary.

- So while it's possible with some current systems to install multiple versions
  of the same package, it's pretty painful to do.

## Being Purely Functional

Nix makes no assumptions about the global state of the system.

This has many advantages, but also some drawbacks. The core of a Nix system is
the Nix store, usually installed under `/nix/store`, and some tools to
manipulate the store.

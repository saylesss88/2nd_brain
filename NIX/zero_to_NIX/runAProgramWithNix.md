---
title: Run a program with Nix
date: 2024-11-30
tags: [tag1, tag2]
---

# Run a program with Nix

```nix
echo "Hello Nix" | nix run "https://flakehub.com/f/NixOS/nixpkgs/*#ponysay"
```

- The first time running `nix run` will take a while because Nix needs to build
  the program's package from scratch--or download it from a known cache--and
  store it in the Nix Store.

## Explaination

What happened here? The Nix CLI did a few things:

- It used the `nixpkgs` flake reference to pull in some Nix code and targeted the `ponysay` flake output (more on this later).

- It built the `ponysay` package and stored the result in the Nix store.

- It ran the executable at `bin/ponysay` from the `ponysay` package.

[[developmentEnvironment.md]]

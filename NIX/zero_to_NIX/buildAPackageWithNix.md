---
title: Build a package with Nix
date: 2024-11-30
tags: [tag1, tag2]
---

# Build a package with Nix

Building bat:

```nix
mkdir build-nix-package && cd build-nix-package
nix build "https://flakehub.com/f/NixOS/nixpkgs/*#bat"
```

- Here, `nixpkgs` is a flake reference to the NixOS/nixpkgs repository on
  GitHub, while `#bat` indicates that we're building the `bat` output from the
  Nixpkgs flake.

- When the build is done, run `ls .` and you should see something called `result`
  in the current directory which is actually a symlink to the built package in the
  Nix store. Verify with:

```nix
readlink result
/nix/store/nxa0p8cg4vp5sh99yv2zwih20xqvd2x0-bat-0.24.0
```

## Build a package for tools written in Python

```nix
nix build "nixpkgs#python3Packages.pip"
./result/bin/pip --help
```

[[buildHomeManager.md]]

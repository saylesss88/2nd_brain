---
date: 12-15-24
aliases: flakes
title: practical nix flakes
---

# Practical Nix Flakes

Flakes are self-contained units that have inputs (dependencies) and outputs
(packages, deployment instructions, Nix functions for use in other flakes).

- Flakes have great reproducibility because they are only allowed to depend on
their inputs and they pin the exact versions of said inputs in a lockfile.

`nix build`: If you want to build something instead of entering a shell with it,
use `nix build`:

```nix
nix build nixpkgs#hello
```

- This will build Hello (or fetch it from the binary cache if available) and
then symlink it to `result` in your current directory.

```nix
./result/bin/hello
Hello, world!
```

`nix develop`: Despite the use of binary caches, Nix is a sourcecode-first
package manager. This means that it has the ability to provide a build
environment for its derivations. So, you can use Nix to manage your build
environments for you! To enter a shell with all runtime and buildtime
dependencies of GNU Hello, use:

```nix
nix develop nixpkgs#hello
```

- Inside that shell, you can call unpackPhase to place GNU Hello sources in
the current directory, then configurePhase to run configure script with correct
arguments and finally buildPhase to build.


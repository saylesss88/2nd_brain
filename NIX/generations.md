---
id: generations
aliases: []
tags:
  - nix generations
---

# Nix Generations

Nix keeps multiple versions of installed packages, and a record of the changes
made. This allows you to easily rollback package management commands. Each
version of your profile after you perform a package manager operation is known
as a _generation_.

You can observe this log of history with the `nix profile history` command.

```nix
> nix profile history
Version 1 (2022-10-05):
  flake:nixpkgs#legacyPackages.x86_64-linux.ripgrep: ∅ -> 13.0.0

Version 2 (2022-10-05) <- 1:
  flake:nixpkgs#legacyPackages.x86_64-linux.fzf: ∅ -> 0.34.0, 0.34.0-man
  flake:nixpkgs#legacyPackages.x86_64-linux.jq: ∅ -> 1.6-bin, 1.6-man
```

- The current version should be highlighted in green.

To rollback changes:

```nix
> nix profile rollback
switching profile from version 2 to 1
```

Run `nix profile history` again and you'll now see version 1 highlighted as the
active version.

Change your mind and want to go back to version 2? The `--to` parameter allows
you to set a specific version as the target:

```nix
> nix profile rollback --to 2
switching profile from version 1 to 2
```

[[profiles.md]]

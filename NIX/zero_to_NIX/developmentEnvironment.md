---
title: Nix Development Environment
date: 2024-11-30
tags: [tag1, tag2]
---

# Nix Development Environment

```nix
nix develop "https://flakehub.com/f/DeterminateSystems/zero-to-nix/*#example"
```

- The `nix develop` command activates a Nix environment.

- This opens a new prompt with preconfigured environment variables.

```nix
type curl
type git
```

## Shell Hooks

Nix lets you define shell hooks, which are shell code that runs whenever the
environment starts up.

```nix
nix develop "github:DeterminateSystems/zero-to-nix#hook"
echo $FUNNY_JOKE
```

To return to your previous environment, use the `exit` command.

### Run commands inside the development environment

The `nix develop` command provides a `--command` (or `-c`) flag that you can use
to run commands that use the environment but from your current shell.

```nix
nix develop "https://flakehub.com/f/DeterminateSystems/zero-to-nix/*#example" --command git help
nix develop "https://flakehub.com/f/DeterminateSystems/zero-to-nix/*#example" --command curl https://example.com
```

- In both cases, you're running a package in the Nix store and nothing from your
  global environment. Nix development environments are completely isolated from
  the surrounding environment (hermetic)

### Language-specific environments

```nix
nix develop "github:DeterminateSystems/zero-to-nix#python"
```

```nix
type python
python -c "print(1 + 1)"
exit
```

Multi-language environments are also supported:

```nix
nix develop "https://flakehub.com/f/DeterminateSystems/zero-to-nix/*#multi"
```

- This Nix environment has Python, kubectl, OpenTofu, OpenSSL.

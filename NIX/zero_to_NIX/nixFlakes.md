---
date: 12-14-24
aliases: flakes
title: nix flakes
---

# Nix Flakes

A Nix Flake is a directory with a flake.nix and a flake.lock at the root that
outputs Nix expressions that others can use to do things like build packages,
run programs, use development environments, or stand up NixOS systems. If 
necessary, flakes can use the outputs of other flakes as inputs.

> [!NOTE] Think of flakes as processors of Nix code. They take Nix expressions
> as input and output things that Nix can use, like package definitions,
> development environments, or NixOS configurations.

## Flake references

A flake reference is a string representation of where the flake is located.
Flake references are used in two places:

1. In flake input declarations to depend on outputs from the flake.

2. In shell environments when running commands like `nix run
   github:DeterminateSystems/flake-checker`

### Flake Inputs

Flake inputs are Nix dependencies that a flake needs to be build. Each input in
the set can be pulled from various sources, such as github, generic git repos,
and your filesystem.

- Furthermore, inputs can modify each other’s inputs to make sure that, for
example, multiple dependencies all rely on the same version of nixpkgs. This
is done via the inputs.<input>.follows attribute.

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };
}
```

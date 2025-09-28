---
title: Flake References
date: 2024-11-30
tags: [Nix, flakes]
---

# Flake References

A _flake reference_ is a string representation of where the flake is located.
Flake references are used in two places:

1. In flake input declarations to depend on outputs from the flake.

2. In shell environments when running commands like `nix run
github:DeterminateSystems/flake-checker`(which runs the flake-checker
   program.)

Examples of flake references:

| Reference                             | Description                                                |
| ------------------------------------- | ---------------------------------------------------------- |
| path:/home/nix-stuff/my-flake         | The /home/nix-stuff/my-flake directory on the current host |
| github:DeterminateSystems/zero-to-nix | The zero-to-nix repository on GitHub                       |

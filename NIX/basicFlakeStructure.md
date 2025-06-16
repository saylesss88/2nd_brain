---
date: 12-15-24
aliases: flakes
title: Basic Flake Structure
---

# Basic Flake Structure

A Nix flake is a directory that contains a `flake.nix` file. That file must
contain an attribute set with one required attribute - `outputs` - and
optionally `description` and `inputs`.

`outputs` is a function that takes an attribute set of inputs (there's always at
least one input - `self` - which refers to the flake that Nix is currently
evaluating; this is possible due to laziness). So, the most trivial flake
possible is this:

```nix
{
  outputs = { self }: { };
}
```

This is a flake with no external inputs and no outputs. Not very useful, huh?

Well, we can add an atbitrary output to it and evaluate it with `nix eval` to
see that it works:

```nix
{
  outputs = { self }: {
    foo = "bar";
  };
}
```

```nix
nix eval .#foo
"bar"
```

Still not very useful.

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
  };

  outputs = { self, nixpkgs }: { };
}
```

---
id: whatAreFlakes
aliases: []
tags: []
---

# What are flakes

Technically, a flake is a file system tree that contains a file named `flake.nix`
in its root directory.

Flakes add the following behavior to Nix:

1. A `flake.nix` file enforces a schema, where:

-   Other flake are referenced as dependencies providing Nix language code or
    other files.
-   The values produced by the Nix expressions in `flake.nix` are structured
    according to pre-defined use cases.

2. References to other flakes can be specified using a dedicated URL-like
   syntax. A _flake registry_ allows using symbolic identifiers for further
   brevity.

3. A new command line interface, implemented as a separate experimental feature,
   leverages flakes by accepting flake references in order to build, run, or
   deploy software defined as a flake.

Nix handles flakes differently than regular Nix files in the following ways:

-   The `flake.nix` file is checked for schema validity. The metadata fields
    cannot be arbitrary Nix expressions. This is to prevent complex, possibly
    non-terminating computations while querying the metadata.

-   The entire flake directory is copied to Nix store before evaluation. This
    allows for effective evaluation caching, which is relevant for large expressions
    such as Nixpkgs, but requires copying the entire flake directory again on each
    change.

-   No external variables, parameters, or impure language values are allowed. It
    means full reproducibility of a Nix expression, and, by extension, the resulting
    build instructions by default, but also prohibits parameterisation of results by
    consumers.

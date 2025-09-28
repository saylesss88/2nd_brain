---
id: flakes
aliases: []
tags:
  - tag1
  - tag2
date: "2024-12-06"
title: NIX flakes
---

# NIX flakes

Technically, a flake is a file system tree that contains a file named
`flake.nix` in its root directory.

Flakes add the following behavior to Nix:

1. A `flake.nix` file enforces a schema, where:

- Other flakes are referenced as dependencies providing Nix language code or
  other files.
- The values produced by the Nix expressions in the `flake.nix` file are
  structured according to pre-defined use cases.

2. References to other flakes can be specified using a dedicated URL-like
   syntax. A flake registry allows using symbolic identifiers for further
   brevity. References can be automatically locked to their current specific
   version and later updated programmatically.

3. A new command line interface, implemented as a separate expirimental feature,
   leverages flakes by accepting flake references in order to build, run, or
   deploy software defined as a flake.

## Paths

When specifying paths in your configuration.nix, it’s important to use relative paths correctly or ensure the path references are valid within the Nix context. Absolute paths might not always work as expected due to Nix's sandboxed nature. Here’s how you can properly structure and import your flake:

1. Relative Paths

If your flake.nix is in a directory like /home/jr/flakes/, ensure you are using a relative path. For example, if your configuration.nix is in /home/jr/, you should reference the flake like this:

```nix
imports = [ ./flakes/flake.nix ];
```

2. Using Flake Inputs

If you want to use flakes properly, you should declare and import them through the inputs section in your flake.nix. Here’s an example:

```nix flake.nix
{
    description = "My flake";

    inputs = {
        nix.pkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    };

    outputs = { self, nixpkgs, ... }: {
        let
          system = "x86_64-linux";
        in {
            nixosConfigurations = {
                myHost = nixpkgs.lib.nixosSystem {
                    inherit system;
                    modules = [
                    ./configuration.nix
                    ./extra-modules/my-extra-config.nix
                    ];
                };
            };
        };
    };
}
```

```nix configuration.nix
{ config, pkgs, ... }:

{
  imports = [
    ./hardware-configuration.nix
    ./extra-modules/my-extra-config.nix  # Example additional module
  ];

  environment.systemPackages = with pkgs; [
    git
    zoxide
    htop
  ];

  # Other configurations
}

```

3. Rebuild Command

Ensure you run the `nixos-rebuild` command with the correct flake path:

```bash
sudo nixos-rebuild switch --flake /home/jr/flakes#myHost
```

If the flake reference is correct, this should properly apply your configuration.

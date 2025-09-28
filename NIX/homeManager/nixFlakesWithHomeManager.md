---
id: nixFlakesWithHomeManager
aliases:
    - nixFlakesWithHomeManager
tags: []
---

# nixFlakesWithHomeManager

You can use the Home Manager flake in 3 ways:

1. Using the standalone home-manager tool. For platforms other than NixOS this
   is the only choice.

2. As a module within a NixOS system configuration. This allows the user
   profiles to be built together with the system when running `nixos-rebuild`.

3. This allows the user profiles to be built together with the system when
   running `darwin-rebuild`.

## Prerequisites

-   Enable `nix-command` and `flakes`.

```nix
nix = {
  package = pkgs.nixFlakes;
  extraOptions = ''
    experimental-features = nix-command flakes
  '';
};
```

-   Alternatively, you can enable flakes on a per-command basis with the following
    additional flags to nix and home-manager:

```nix
nix --experimental-features "nix-command flakes" <sub-commands>
home-manager --experimental-features "nix-command flakes" <sub-commands>
```

-   Prepare your Home Manager configuration (`home.nix`).

Unlike the channel based setup, `home.nix` will be evaluated when the flake is
built, so it must be present before bootstrap of Home Manager from the flake.

## NixOS module

To use Home Manager as a NixOS module, a bare-minimum `flake.nix` would be:

```nix
{
  description = "NixOS configuration";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    home-manager.url = "github:nix-community/home-manager";
    home-manager.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs@{ nixpkgs, home-manager, ... }: {
    nixosConfigurations = {
      hostname = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [
          ./configuration.nix
          home-manager.nixosModules.home-manager
          {
            home-manager.useGlobalPkgs = true;
            home-manager.useUserPackages = true;
            home-manager.users.jdoe = import ./home.nix;

            # Optionally, use home-manager.extraSpecialArgs to pass
            # arguments to home.nix
          }
        ];
      };
    };
  };
}
```

The Home Manager configuration is then part of the NixOS configuration and is
automatically rebuilt with the system when using the appropriate command for
the system, such as nixos-rebuild switch --flake <flake-uri>.

You can use the above flake.nix as a template in /etc/nixos by

```nix
nix flake new /etc/nixos -t github:nix-community/home-manager#nixos
```

[[writingHomeManagerModules.md]]

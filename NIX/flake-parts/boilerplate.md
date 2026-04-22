# Flake parts boilerplate

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ { flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; }
    {
    # Supported Systems
    systems = [ "x86_64-linux" "x86_64-darwin"];
    # like regular `self` but with `system` pre-selected
    perSystem = { pkgs, self', ... }: {
      packages.mypackage = pkgs.sl;
      devShells.default = pkgs.mkShell {
        packages = [ self'.packages.mypackage];
      };

    };

    };
}
```

More Modular

`package.nix`

```nix
{
  perSystem = { pkgs, ...}: {
    packages.mypackage = pkgs.sl;
  };
}
```

`shell.nix`

```nix
{
  perSystem = { pkgs, self', ...}: {
  devShells.default = pkgs.mkShell {
    packages = [ self'.packages.mypackage ];
  };
};
}
```

`flake.nix`

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ { flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; }
    {
      imports = [ ./shell.nix ./package.nix ./nixos.nix ]
    };

}
```

`nixos.nix`

```nix
{ inputs, ... }:
{
  flake = {

    nixosConfigurations.main = inputs.nixpkgs.lib.nixosSystem {
      modules = [
        ./configuration.nix
      ];
    };
  };
}
```

```sh
# option type: attrset
flake.nixosConfigurations.<name>

# option type: module
flake.nixosModules.<name>

# option type: package
perSystem.packages.<name>

# option type: package
perSystem.devShells.<name>
```

## Flake-Parts Modules

```nix
{ self, inputs, config, ... }: {
  flake.nixosModules.myNixOSModule =
    { pkgs, ... }: {

      imports = [
        inputs.self.nixosModules.default
      ];

      environment.systemPackages = [
        pkgs.kitty
        pkgs.pcmanfm
      ];
    };
}
```

```nix
{ self, inputs, config, ... }: {
  flake.nixosModules.myNixOSModule =
    { pkgs, ... }: {

      hardware.bluetooth.enable = true;

      fonts.packages = [
        pkgs.nerd-fonts.jetbrains-mono
      ];
    };
}
```

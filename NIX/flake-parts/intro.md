# flake-parts intro

`flake-parts` provides the options that represent standard flake attributes and
establishes a way of working with `system`. Opinionated features are provided by
an ecosystem of modules that you can import.

Standard flake:

```nix
{
  inputs = {
    nixpkgs.url = "github.nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }: let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in {
    packages.x86_64-linux.mypackage = pkgs.sl;

    devShells.x86_64-linux.default = pkgs.mkShell {
      packages = [ self.packages.x86_64-linux.mypackage ];
    };
  };
}
```

`flake-parts` boilerplate:

```nix
{
  inputs = {
    nixpkgs.url = "github.nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ { self, nixpkgs, ... }: let
    flake-parts.lib.mkFlake { inherit inputs ; }
    {
      systems = [ "x86_64-linux" "x86_64-darwin" ];
    };
}
```

Add `perSystem`, responsible for system-specific outputs.

```nix
{
  inputs = {
    nixpkgs.url = "github.nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ { self, nixpkgs, ... }: let
    flake-parts.lib.mkFlake { inherit inputs ; }
    {
      imports = [ ];
      systems = [ "x86_64-linux" "x86_64-darwin" ];
      perSystem = { pkgs, self', ... }: {
        packages.mypackage = pkgs.sl;
        devShells.default = pkgs.mkShell {
          packages = [ self'.packages.mypackage ];
        };
      };
    };
}
```

Now we can modularize the flake using the top-level `imports`:

```nix
{
  inputs = {
    nixpkgs.url = "github.nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ { self, nixpkgs, ... }: let
    flake-parts.lib.mkFlake { inherit inputs ; }
    {
      imports = [ ./package.nix ./shell.nix ./nixos.nix ]
      systems = [ "x86_64-linux" "x86_64-darwin" ];
      perSystem = { pkgs, self', ... }: {
        packages.mypackage = pkgs.sl;
        devShells.default = pkgs.mkShell {
          packages = [ self'.packages.mypackage ];
        };
      };
    };
}
```

`nixosConfigurations` don't live in a system specific attribute so we will put
ours in `flake` instead of `perSystem`. In the end `flake` will expose
everything at the top-level of the outputs. Since the `nixosSystem` function
comes from `nixpkgs` we'll also request `inputs` in the parameter set at the
top.

`nixos.nix`

```nix
{ inputs, ... }: {
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

## `_module.args`

In `flake-parts`, `_module.args` is your way of **injecting custom
dependencies** into the module system so that every other part of your
configuration can "see" them as arguments.

You use it specifically when you want to override the default behavior of how
`flake-parts` provides arguments to your `perSystem` blocks.

By default, `flake-parts` provides a basic `pkgs` derived from `inputs.nixpkgs`.
If you need unfree packages or overlays, the default `pkgs` won't work.

Using `_module.args.pkgs` tells `flake-parts`: "Don't use the standard
`nixpkgs`; use this specific instance I just built."

Configure `pkgs` in `perSystem` and reuse it in your NixOS configurations using
`withSystem`.

In a flake-parts module (e.g., `./nixos.nix`):

```nix
{ withSystem, inputs, ... }: {
  perSystem = { system, ... }: {
    _module.args.pkgs = import inputs.nixpkgs {
      inherit system;
      overlays = [ inputs.foo.overlays.default ];
      config = {
        allowUnfree = true;
      };
    };

    # Now use this configured pkgs in your packages, devShells, etc.
    packages.my-package = pkgs.hello;
  };

  flake.nixosConfigurations.my-machine = inputs.nixpkgs.lib.nixosSystem {
    modules = [
      ./configuration.nix
      inputs.nixpkgs.nixosModules.readOnlyPkgs
      ({ config, ... }: {
        # Use the configured pkgs from perSystem
        nixpkgs.pkgs = withSystem config.nixpkgs.hostPlatform.system (
          { pkgs, ... }: # perSystem module arguments
          pkgs
        );
      })
    ];
  };
}
```

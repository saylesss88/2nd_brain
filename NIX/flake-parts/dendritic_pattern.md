# Dendritic Pattern

```nix
{
  inputs = {
    nixpkgs.url = "github.nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    import-tree.url = "github:vic/import-tree";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs ; }
      (inputs.import-tree ./modules);
}
```

`myFirstModule.nix`:

```nix
{inputs, ... }: {
  flake.nixosModules.myFirstModule = { pkgs, ... }: {

    programs.firefox.enable = true;

    environment.systemPackages = with pkgs; [
      vim
    ];
  };
}
```

`myMachine.nix`:

```nix
{ inputs, self, ... }: {
  flake.nixosConfigurations.myMachine = inputs.nixpkgs.lib.nixosSystem {
    modules = [
      self.nixosModules.myMachineModule
      self.nixosModules.myFirstModule
    ];

  };

  flake.nixosModules.myMachineModule = { pkgs, ... }: {
    boot.loader.grub.enable = true;
  };
}
```

`myPackage.nix`:

```nix
{ inputs, ... }: {
{
  perSystem = { pkgs, ...}: {
    packages.myPackage = pkgs.stdenv.mkDerivation {
      pname = "myPackage";
      version = "1.0.0";
    };
  };
};
}
```

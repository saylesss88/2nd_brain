---
id: packageManagement
aliases: []
tags:
    - nix package_management
---

# Package Management

## Declarative Package Management

```nix
{
  environment.systemPackages = [ pkgs.thunderbird ];
}
```

-   The thunderbird package from Nixpkgs will be built or downloaded as part of
    the system when you run `nixos-rebuild switch`.

You can get a list of the available packages as follows:

```nix
nix-env -qaP '*' --description
```

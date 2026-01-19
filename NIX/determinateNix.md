# Determinate Nix

1. Flake input

```nix
inputs = {
determinate.url = "https://flakehub.com/f/DeterminateSystems/determinate/*";
}
```

2.

```nix
modules = [
  inputs.determinate.nixosModules.default
];
```

3. Initial rebuild:

```bash
# one time for cache
sudo nixos-rebuild \
  --option extra-substituters https://install.determinate.systems \
  --option extra-trusted-public-keys cache.flakehub.com-3:hJuILl5sVK4iKm86JzgdXW12Y2Hwd5G07qKtHTOcDCM= \
  --flake .#magic \
  switch
# sudo nixos-rebuild switch --flake .#magic --option extra-substituters https://install.determinate.systems --option extra-trusted-public-keys cache.flakehub.com-3:hJuIL15sVK4iKm86JzgdXW12Y2Hwd5G07qKtHTOcDCM=
```

4.

```bash
sudo determinate-nixd upgrade
determinate-nixd status
determinate-nixd auth login
```

5. `configuration.nix`:

```nix
environment.etc."nix/nix.custom.conf".text = ''
  extra-substituters = https://cache.flakehub.com
  extra-trusted-public-keys = cache.flakehub.com
```

### Resources

- [Determinate on NixOS](https://docs.determinate.systems/guides/advanced-installation/#install-on-nixos)

## Nix-Toolbox

System-wide trust:

```bash
run0 podman image trust set -t accept ghcr.io/thrix/nix-toolbox:42
```

Create Nix Toolbox container:

```bash
distrobox create --image ghcr.io/thrix/nix-toolbox:42
```

Enter the toolbox:

```bash
distrobox enter nix-toolbox-42
```

---
id: addingModulesFromThirdPartyFlakes
aliases:
    - addingModulesFromThirdPartyFlakes
tags: []
---

# addingModulesFromThirdPartyFlakes

One of the main goals of flake-based NixOS is to make it easier to use packages
and modules that aren't included in the `nixpkgs` repository. As an example,
we'll add Hydra (a continuous integration server) to our container.

To add it to our container, we specify it as an additional input:

```nix
inputs.hydra.url = "github:NixOS/hydra";
```

and as a corresponding function argument to the `outputs` function:

```nix
outputs = { self, nixpkgs, hydra }: {
```

Finally, we enable the NixOS module provided by the `hydra` flake:

```nix
 modules =
        [ hydra.nixosModules.hydraTest

          ({ pkgs, ... }: {
            ... our own configuration ...

            # Hydra runs on port 3000 by default, so open it in the firewall.
            networking.firewall.allowedTCPPorts = [ 3000 ];
          })
        ];
```

Note that we can discover the name of this module by using nix flake show:

```nix
nix flake show github:NixOS/hydra
github:NixOS/hydra/d0deebc4fc95dbeb0249f7b774b03d366596fbed
├───…
├───nixosModules
│   ├───hydra: NixOS module
│   ├───hydraProxy: NixOS module
│   └───hydraTest: NixOS module
└───overlay: Nixpkgs overlay
```

After committing this change and running nixos-container update, we can check
whether hydra is working in the container by visiting http://flake-test:3000/
in a web browser.

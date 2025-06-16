---
id: usingFlakesForNixOSConfigurations
aliases: []
tags:
    - flakes
---

# Using flakes for NixOS configurations

## Creating a NixOS Configuration Flake

```bash
git init my-flake
cd my-flake
nix flake init -t templates#simpleContainer
git commit -a -m "Initial version"
```

-   The `-t` flag to `nix flake init` specifies the template from which to copy
    the initial contents of the flake.

To see what templates are available, you can run:

```nix
nix flake show templates
```

The initial flake looks like this:

```nix
{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-20.03";

  outputs = { self, nixpkgs }: {

    nixosConfigurations.container = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules =
        [ ({ pkgs, ... }: {
            boot.isContainer = true;

            # Let 'nixos-version --json' know about the Git revision
            # of this flake.
            system.configurationRevision = nixpkgs.lib.mkIf (self ? rev) self.rev;

            # Network configuration.
            networking.useDHCP = false;
            networking.firewall.allowedTCPPorts = [ 80 ];

            # Enable a web server.
            services.httpd = {
              enable = true;
              adminAddr = "morty@example.org";
            };
          })
        ];
    };

  };
}
```

-   The flake has one input, `nixpkgs` specifically the 20.03 branch.

-   It has one output, `nixosConfigurations.container`, which evaluates a NixOS
    configuration for tools like `nixos-rebuild` and `nixos-container`.

-   The main argument is `modules`, which is a list of NixOS configuration
    modules. This takes the place of the `configuration.nix` in non-flake
    deployments. You can write `modules = [ ./configuration.nix ]` if you're
    converting a pre-flake NixOS configuration.

Let's start the container!:

```nix
sudo nixos-container create flake-test --flake /home/jr/my-flake
sudo nixos-container start flake-test
```

And check whether the container works:

```bash
curl http://flake-test/
<html><body><h1>It works!</h1></body></html>
```

If you wnat to build the container without the `nixos-container` command, you
can use:

```nix
nix build
/home/jr/my-flake#nixosConfigurations.container.config.system.build.toplevel
```

> [!NOTE]: `system.build.toplevel` is an internal NisOS option that evaluates
> to the "system" derivation that commands like `nixos-rebuild`,
> `nixos-install`, and `nixos-container` build and activate.

### Hermetic Evaluation

Flake-based NixOS Systems record the Git revisions from which they were built.
We can query this as follows:

```bash
sudo nixos-container run flake-test -- nixos-version --json
{"configurationRevision":"9190c396f4dcfc734e554768c53a81d1c231c6a7"
,"nixosVersion":"20.03.20200622.13c15f2"
,"nixpkgsRevision":"13c15f26d44cf7f54197891a6f0c78ce8149b037"}
```

-   `configurationRevision` is the Git revision of the repo `home/jr/my-flake`.

If we want to deploy this particular configuration to a container, we can:

```bash
sudo nixos-container update flake-test \
  --flake /path/to/my-flake?rev=9190c396f4dcfc734e554768c53a81d1c231c6a7
```

[[addingModulesFromThirdPartyFlakes.md]]

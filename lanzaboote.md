# Lanzaboote

1. Ensure you are using a UEFI system with systemd-boot with:

```bash
sudo bootctl status
```

Verify that `/boot` is the ESP by looking for `ESP:` in the output of
`bootctl status`:

```bash
Available Boot Loaders on ESP:
          ESP: /boot (/dev/disk/by-partuuid/f29c5e71-e371-4224-aeb9-42c4142474dc)
         File: ├─/EFI/systemd/systemd-bootx64.efi (systemd-boot 257.5)
               └─/EFI/BOOT/BOOTX64.EFI (systemd-boot 257.5)
```

2. Install `sbctl` from nixpkgs:

```bash
# configuration.nix
environment.systemPackages = [ pkgs.sbctl ];
```

3. Create your keys:

```bash
sudo sbctl create-keys
[sudo] password for julian:
Created Owner UUID 8ec4b2c3-dc7f-4362-b9a3-0cc17e5a34cd
Creating secure boot keys...✓
Secure boot keys created!
```

4. Configure NixOS (with Flakes):

```nix
{
  description = "A SecureBoot-enabled NixOS configurations";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    lanzaboote = {
      url = "github:nix-community/lanzaboote/v0.4.2";

      # Optional but recommended to limit the size of your system closure.
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, lanzaboote, ...}: {
    nixosConfigurations = {
      yourHost = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";

        modules = [
          # This is not a complete NixOS configuration and you need to reference
          # your normal configuration here.

          lanzaboote.nixosModules.lanzaboote

          ({ pkgs, lib, ... }: {

            environment.systemPackages = [
              # For debugging and troubleshooting Secure Boot.
              pkgs.sbctl
            ];

            # Lanzaboote currently replaces the systemd-boot module.
            # This setting is usually set to true in configuration.nix
            # generated at installation time. So we force it to false
            # for now.
            boot.loader.systemd-boot.enable = lib.mkForce false;

            boot.lanzaboote = {
              enable = true;
              pkiBundle = "/var/lib/sbctl";
            };
          })
        ];
      };
    };
  };
}
```

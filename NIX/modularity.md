---
id: modularity
aliases:
    - modularity
tags: []
---

# modularity

Modules have the same syntax as `configuration.nix`. In fact, `configuration.nix`
is a module itself. You can use modules by including them from `configuration.nix`:

```nix
{ config, pkgs, ... }:

{ imports = [ ./vpn.nix ./kde.nix ];
  services.httpd.enable = true;
  environment.systemPackages = [ pkgs.emacs ];
  # ...
}
```

Here, we include two modules from the same directory, `vpn.nix` and `kde.nix`:
The latter might look like this:

```nix
{ config, pkgs, ... }:

{ services.xserver.enable = true;
  services.displayManager.sddm.enable = true;
  services.xserver.desktopManager.plasma5.enable = true;
  environment.systemPackages = [ pkgs.vim ];
}
```

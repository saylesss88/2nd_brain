---
id: mkpasswd
aliases: []
tags: []
---

# mkpasswd

Either install `mkpasswd` or run:

```nix
nix-shell -p mkpasswd --run "mkpasswd -m sha-512"
```

Then add the Hashed Password to your NixOS configuration:

```nix
{ config, pkgs, ... }:
{
  users.users.<your-username> = {
    isNormalUser = true;
    home = "/home/<your-username>";
    description = "<Your Name>";
    extraGroups = [ "wheel" ]; # Enable ‘sudo’ for the user.
    hashedPassword = "<your-hashed-password-here>";
  };
}
```

-   This allows for declarative management of your user password making
    reproducing the setup easier.

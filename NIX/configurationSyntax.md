---
id: configurationSyntax
aliases:
    - configurationSyntax
tags: []
---

# configurationSyntax

The Nixos configuration file generally looks like this:

```nix
{ config, pkgs, ...}:

{ /* option definitions */
}
```

The first line `{ config, pkgs, ...}:` denotes that this is actually a function
that takes at least two arguments `config` and `pkgs`.

The function returns a set of option definitions `{ ... }`. These definitions
have the form `name = value`, where `name` is the name of an option and `value`
is the value. For example:

```nix
{ config, pkgs, ... }:

{ services.httpd.enable = true;
  services.httpd.adminAddr = "alice@example.org";
  services.httpd.virtualHosts.localhost.documentRoot = "/webroot";
}
```

defines a configuration that has three option definitions that together enable
the Apache HTTP Server with `/webroot` as the document root.

Sets can be nested, and in fact dots in option names are shorthand for defining
a set containing another set. `services.httpd.enable` defines a set named
`services` that contains a set named `httpd` that contains a set named `enable`
with a value `true`. So above can be written as:

```nix
{ config, pkgs, ... }:

{ services = {
  httpd = {
    enable = true;
    adminAddr = "alice@example.org";
    virtualHosts = {
      localhost = {
        documentRoot = "/webroot";
      };
    };
  };
};
```

which may be more convenient if you have lots of option definitions that share
the same prefix like `services.httpd`

## Sets

Sets are name/value pairs enclosed in braces, as in the option definition.

```nix
{
  fileSystems."/boot" =
  { devices = "/dev/sda1";
    fsType = "ext4";
    options = [ "rw" "data=ordered" "relatime" ];
  };
}
```

## Packages

Usually, the packages you need are already part of the Nix Packages collection,
which is a set that can be accessed through the function argument pkgs. Typical
uses:

```nix
{
  environment.systemPackages =
    [ pkgs.thunderbird
      pkgs.emacs
    ];

  services.postgresql.package = pkgs.postgresql_14;
}
```

### Multiple modules

When using multiple modules, you may need to access configuration values defined
in other modules.

That is what the `config` function argument is for: it contains the complete,
merged system configuration. That is `config` is the result of combining the
configurations returned by every module.

For example, here is a module that adds some packages to environment.systemPackages
only if services.xserver.enable is set to true somewhere else:

```nix
{ config, pkgs, ... }:

{ environment.systemPackages =
    if config.services.xserver.enable then
      [ pkgs.firefox
        pkgs.thunderbird
      ]
    else
      [ ];
}
```

[[abstractions.md]]

---
id: modules
aliases: []
tags: []
---

# Modules

A _module_ is a piece of Nix code that you can configure to produce
configuration. In NixOS, a module is a function that takes an attribute set and
returns another attribute set.

The basic anatomy of a module is as follows:

```nix
{ lib, config, pkgs, ... }:

{
  imports = [];
  config = {};
  options = {};
}
```

## How to use modules

You can find modules and how to use them via the NixOS module search.

The following is an example of how to setup and configure the nginx web server:

```nix
{ ... }:

{
  services.nginx = {
    enable = true;
    virtualHosts."default" = {
      forceSSL = true;             # Redirect HTTP clients to HTTPS
      default = true;              # Always use this host, no matter hostname
      root = /var/www/my-website;  # Set the web root to serve
    };
  };
}
```

<details>
<summary> ✔️ Alternative LibreWolf Configuration utilizing Arkenfox </summary>

```nix
{
  pkgs,
  lib,
  config,
  ...
}: let
  cfg = config.custom.librewolf;
in {
  options.custom.librewolf = {
    enable = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "Enable the LibreWolf Module";
    };
  };

  config = lib.mkIf cfg.enable {
    programs.librewolf = {
      enable = true;
      policies = {
        DontCheckDefaultBrowser = true;
        DisablePocket = true;
        DisableAppUpdate = true;
      };
      profiles.default = {
        isDefault = true;
        name = "Default Profile";
        extraConfig = ''
          ${builtins.readFile ./user.js}
        '';

        settings = {
          # Use dnscrypt
          # "network.trr.mode" = 2;
          # "network.trr.uri" = "https://127.0.0.1:3000/dns-query";

          "general.autoScroll" = true;
          "sidebar.verticalTabs" = true;
        };
      };
    };
    xdg.desktopEntries.librewolf = {
      name = "LibreWolf";
      exec = "${pkgs.librewolf}/bin/librewolf";
    };
    xdg.mimeApps = {
      enable = true;
      defaultApplications = {
        "text/html" = "librewolf.desktop";
        "x-scheme-handler/http" = "librewolf.desktop";
        "x-scheme-handler/https" = "librewolf.desktop";
        "x-scheme-handler/about" = "librewolf.desktop";
        "x-scheme-handler/unknown" = "librewolf.desktop";
      };
    };
  };
}
```

In the Firefox section I better explain how to set up the `user.js` required for
Arkenfox. I have been able to use the same `profiles.default` for both LibreWolf
and Firefox.

</details>

### Arkenfox

If you want to use this, it requires adding `firefox-addons` to your `flake.nix`

```nix
# firefox.nix
{
  config,
  inputs,
  pkgs,
  ...
}: let
  # Extract the firefoxAddons package set from the flake input for your system
  firefoxAddons = inputs.firefox-addons.packages."x86_64-linux";
in {
  programs.firefox = {
    enable = true;

    profiles.default = {
      isDefault = true;
      name = "Default Profile";
      extraConfig = ''
        ${builtins.readFile ./user.js}
      '';
      settings = {
        # Example, this is already set
        # "privacy.resistFingerprinting" = true;
      };
      extensions.packages = with firefoxAddons; [ublock-origin];
    };
  };
}
```

Download the
[Arkenfox user.js](https://github.com/arkenfox/user.js/blob/master/user.js) and
review it making sure that you agree with the settings. If you do, place it in
the same directory as your `firefox.nix`.

You will have to go through the entire `user.js` and uncomment certain settings
that you want to apply such as enabling RFP by uncommenting the following:

```js
user_pref("privacy.resistFingerprinting", true); // [FF41+]
user_pref("privacy.resistFingerprinting.pbmode", true); // [FF114+]
```

As you learn more, you can get more strict if you so choose.

Rebuild, launch Firefox, and check your `~/.mozilla/firefox/default/user.js`. It
should match the Arkenfox settings. The `prefs.js` should also match.

In Firefox type `Ctrl + Shift + J` and look for any errors. I had to create a
`mkdir -p ~/.mozilla/firefox/default/thumbnails` to remove a warning.

Type `about:config` into the address bar and search a few of the settings that
Arkenfox changes, do they match?

Read the [Arkenfox Wiki](https://github.com/arkenfox/user.js/wiki)

### Firefox

Firefox's defaults are not as privacy respecting as LibreWolf's but can be
manually configured to be fairly private and secure.

- Switch to a privacy respecting search engine such as duckduckgo.

- Add some privacy respecting search engines in `about:preferences#search` at
  the bottom of the screen click `Add`. A few good ones are `searx` and
  `startpage`:
  - Search engine name: `SearXNG`, URL: `https://searx.be/search?q=%s`

  - Search engine name: `StartPage`, URL:
    `https://www.startpage.com/do/search?q=%s`

- Disable search suggestions

- In `about:preferences#search`, disable Address Bar suggestions. Disabls Search
  Suggestions.

- Disable all Firefox Data Collection. Disabls Search Suggestions.

- Disable all Firefox Data Collection

- Ensure that HTTPS-Only Mode is set in all windows

- Avoid untrusted extensions!

- `about:config` set `fission.autostart` to true.

---
id: writing_packages
aliases: []
tags: []
---

# Contributing to Nixpkgs

```bash
cd ~/src/nixpkgs
gh issue list
gh issue list --label "0.kind: packaging request" --json id
gh issue list --label "0.kind: packaging request" --json url | jq '.[].url'
gh issue list --label "0.kind: packaging request" --json url | jq -r '.[].url'
gh issue list --limit 2000 --label "0.kind: packaging request" --json url | jq -r '.[].url' > requests
 cat requests | wc
   1000    1000   47000
gst
On branch master
Your branch is up to date with 'origin/master'.

Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
        new file:   overlay.nix
        new file:   test2.nix

Changes not staged for commit:
  (use "git add/rm <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        deleted:    overlay.nix
        modified:   pkgs/build-support/rust/build-rust-package/default.nix
        modified:   pkgs/by-name/ai/airshipper/package.nix
        modified:   pkgs/by-name/jq/jq/package.nix

Untracked files:
  (use "git add <file>..." to include in what will be committed)
        airshipper.nix
        requests
        test.nix
        yad.nix
git stash -u   # Now we have a clean directory
git clean -f -x -d ..
git clean -f -x -d .
git pull      # pull from master
rg 'promethious-'
```

Create a script doing the same for the future:

```bash
#!/usr/bin/env bash
gh -R NixOS/nixpkgs issue list --limit 2000 --label "0.kind: packaging request" --json url | jq -r '.[].url' > requests
```

Get a random request:

```bash
bat requests | shuf | head -1
# Ctrl + Enter will launch the issue
https://github.com/NixOS/nixpkgs/issues/359633
shuf requests | head -1
```

`all-packages.nix` is the top-level package collection of nixpkgs. It's sorted
by categories corresponding to the folder names in the `/pkgs` folder.

```bash
hx pkgs/top-level/all-packages.nix
```

With such a large file it can be slow with syntax highlighting enabled. For
neovim you can disable it while in the file with `:TSDisable highlight`

```bash
rq prometheus-
```

Long list, pick one to see where exporters are implemented:

```bash
❯ rg mikrotik-exporter
pkgs/top-level/all-packages.nix
10859:  prometheus-mikrotik-exporter =
10860:    callPackage ../servers/monitoring/prometheus/mikrotik-exporter.nix {};

nixos/tests/prometheus-exporters.nix
813:        wait_for_unit("prometheus-mikrotik-exporter.service")

pkgs/servers/monitoring/prometheus/mikrotik-exporter.nix
9:  pname = "mikrotik-exporter-unstable";
14:    repo = "mikrotik-exporter";
28:    mainProgram = "mikrotik-exporter";
```

We can see that 10859: `prometheus-mikrotik-exporter` is a top-level package
located at `pkgs/top-level/all-packages.nix` but also the number of spaces after
the `:` only 2 spaces indicates a top-level package and that it's not nested.

We will add `qbittorrent-exporter.nix` added below pve-exporter.nix

```nix
# all-packages.nix
  prometheus-pve-exporter =
    callPackage ../servers/monitoring/prometheus/pve-exporter.nix {};

  prometheus-qbittorrent-exporter =
    callPackage ../servers/monitoring/prometheus/qbittorrent-exporter.nix {};
```

To see where else exporters are implemented:

```bash
rg prometheus-pve-exporter
pkgs/top-level/all-packages.nix
10891:  prometheus-pve-exporter =

nixos/tests/prometheus-exporters.nix
1315:          wait_for_unit("prometheus-pve-exporter.service")

nixos/doc/manual/release-notes/rl-2205.section.md
136:- [prometheus-pve-exporter](https://github.com/prometheus-pve/prometheus-pve-exporter), a tool that exposes information from the Proxmox VE API for use by Prometheus. Available as [services.prometheus.exporters.pve](#opt-services.prometheus.exporters.pve.enable).

pkgs/servers/monitoring/prometheus/pve-exporter.nix
37:    homepage = "https://github.com/prometheus-pve/prometheus-pve-exporter";

nixos/modules/services/monitoring/prometheus/exporters/pve.nix
30:    package = mkPackageOption pkgs "prometheus-pve-exporter" { };
35:      example = "/etc/prometheus-pve-exporter/pve.env";
41:        Environment reference: https://github.com/prometheus-pve/prometheus-pve-exporter#authentication
48:      example = "/etc/prometheus-pve-exporter/pve.yml";
56:        Configuration reference: https://github.com/prometheus-pve/prometheus-pve-exporter/#authentication
64:        example = "/var/lib/prometheus-pve-exporter/privkey.key";
73:        example = "/var/lib/prometheus-pve-exporter/full-chain.pem";
```

We can see that it's implemented in
`nixos/modules/services/monitoring/prometheus/exporters/`

```bash
hx nixos/modules/services/monitoring/prometheus/exporters/ <TAB>
```

It looks like each exporter defines its own module as well. You can open any of
them to get an idea of the work involved.

`:! touch nixos/modules/services/monitoring/prometheus/qbittorrent-exporter.nix`

```bash
hx pkgs/top-level/all-packages.nix
:! touch pkgs/servers/monitoring/prometheus/qbittorrent-exporter.nix
# Then scroll to our callPackage where the path starts and type gf
# qbittorrent-exporter.nix will open in the editor buffer
```

```nix
# qbittorrent-exporter.nix
throw "foo"
```

Before we start well check that we can actually use it:

```bash
nix-build -A prometheus-qbittorrent-exporter
       (stack trace truncated; use '--show-trace' to show the full, detailed trace)

       error: foo
```

Ok it works!

Looking at it it's a python package so we can look at the other exporters to see
if they're all python packages we might be able to copy something from there.

```bash
cd nixos/modules/services/monitoring/prometheus/exporters/
# or `zi`: exporters for zoxide
rg python -i
py-air-control.nix
49:        ${pkgs.python3Packages.py-air-control-exporter}/bin/py-air-control-exporter \
hx $(ls | shuf | head -1)
```

Need to look at the prometheus repo to see if the output is a binary or a python
package.

Looking at the repo it's installed with pip and can be run with
`qbittorrent-exporter` so it seems to be a binary.

```nix
# qbittorrent-exporter.nix
{ buildPythonApplication }:
buildPythonApplication {

}
```
Save and try building to see if the compiler tells us if we're on the right
track.

```bash
cd ~/src/nixpkgs
nix-build -A promethius-qbittorrent-exporter
       error: evaluation aborted with the following error message: 'lib.customisation.callPackageWith: Function called without required argument "buildPythonApplication" at /home/jr/src/nixpkgs/pkgs/servers/monitoring/prometheus/qbittorrent-exporter.nix:1'
```

```nix
{ python3Packages }:
python3Packages.buildPythonApplication {

}
```

```bash
nix-build -A promethius-qbittorrent-exporter
      error: attribute 'pname' missing
       at /home/jr/src/nixpkgs/pkgs/development/interpreters/python/mk-python-derivation.nix:285:44:
          284|
          285|       name = namePrefix + attrs.name or "${finalAttrs.pname}-${finalAttrs.version}";
             |                                            ^
          286|
       Did you mean name?
```

Ok a new error `pname ` missing.

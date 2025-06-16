---
date: 12-14-24
aliases: Nixos
title: Nix Cheat Sheet
---

# Nix Cheat Sheet

Get the store path for a package

```nix
nix repl
nix-repl> :l <nixpkgs>
Added 7486 variables.
nix-repl> "${xorg.libXtst}"
"/nix/store/nlpnx21yjdjx2ii7ln4kcmbm0x1vy7w9-libXtst-1.2.3"

nix-repl> :lf ./configuration.nix # as flakes way for a local file

# load nixos configuration from a nix file
nix repl --file '<nixpkgs/nixos>' -I nixos-config=./configuration.nix

nix-build '<nixpkgs>' --no-build-output -A xorg.libXtst
/nix/store/nlpnx21yjdjx2ii7ln4kcmbm0x1vy7w9-libXtst-1.2.3
```

## Adding files to the store

```nix
nix-store --add-fixed sha256 /path/to/file
```

- Unfortunately, `nix-store` will try to load the entire file into memory, which
  will fail if the file size exceeds available memory. If we have root access,
we can copy the file to the store ourselves:

```nix
$ sudo unshare -m bash  # open a shell as root in a private mount namespace
$ largefile=/path/to/file
$ hash=$(nix-hash --type sha256 --flat --base32 $largefile)  # sha256 hash of the file
$ storepath=$(nix-store --print-fixed-path sha256 $hash $(basename $largefile))  # destination path in the store
$ mount -o remount,rw /nix/store  # remount the store in read/write mode (only for this session)
$ cp $largefile $storepath  # copy the file
$ printf "$storepath\n\n0\n" | nix-store --register-validity --reregister  # register the file in the Nix database
$ exit  # exit to the original shell where /nix/store is still mounted read-only
```

### Build nixos from nixpkgs repo

The following will build the system from a git checkout:

```nix
nixos-rebuild -I nixpkgs=/path/to/nixpkgs switch
```

- This method can be used when testing nixos services for a pull request to
nixpkgs.

Use the following command to build directly from a particular branch of a repository in GitHub:

```nix
$ nixos-rebuild -I nixpkgs=https://github.com/nixcloud/nixpkgs/archive/release-17.03.tar.gz switch
```

#### Evaluate a NixOS configuration without building

If you only want to evaluate `configuration.nix` without building, you can use:

```nix
nix-instantiate '<nixpkgs/nixos' -A system
```
- This creates the `.drv` file that `nixos-rebuild build` would build.

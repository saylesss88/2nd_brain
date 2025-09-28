---
title: pacman Tips
date: 2024-10-31
tags: [tag1, tag2]
---

# Pacman Tips

spelling is wrong.

Keeping a list of all explicitly installed packages can be useful to backup a
system or quicken the installation of a new one:

```bash
pacman -Qqe > pkglist.txt
```

Use `comm -13 <(pacman -Qqdt | sort) <(pacman -Qqdtt | sort) > optdeplist.txt`
to also create a list of the installed optional dependencies which can be
reinstalled with `--asdeps`

Use `pacman -Qqem > foreignpkglist.txt` to create a list of AUR and other
foreign packages that have been explicitly installed.

## Create a hook to keep the list up to date

To keep an up-to-date list of all explicitly installed packages (eg. in
combination with a versioned `/etc/`), you can set up a hook:

```bash
[Trigger]
Operation = Install
Operation = Remove
Type = Package
Target = *

[Action]
When = PostTransaction
Exec = /bin/sh -c '/usr/bin/pacman -Qqe > /etc/pkglist.txt'
```

### Install packages from a list

```bash
sudo pacman -S --needed - < pkglist.txt
```

It is likely foreign packages AUR are present in the list. To filter out from
the list the foreign packages, the previous command can be enriched as follows:

```bash
sudo pacman -S --needed $(comm -12 <(pacman -Slq | sort) <(sort pkglist.txt))
```

Eventually, to make sure the installed packages of your system match the list
and remove all the packages that are not mentioned in it:

```bash
sudo pacman -Rsu $(comm -23 <(pacman -Qq | sort) <(sort pkglist.txt))
```

- These can be automated, see `bacpac`, `pacup`, `pacmanity`, and `pug` (AUR)

### Reinstall all packages:

To reinstall all native packages, use:

```bash
sudo pacman -Qqn | pacman -S -
```

Foreign AUR packages must be reinstalled separately; you can list them with
`pacman -Qqm`

> [!WARNING] To force all packages to be overwritten, use `--overwrite=*`,
> though this should be an absolute last resort.

### Clean Cache Smartly

Use `paccache` to keep only a few recent package versions and free disk space
safely:

```bash
sudo paccache -r
sudo paccache -ruk0  # Remove all except currently installed
```

### ILoveCandy

Add the following to your `/etc/pacman.conf`:

```bash
Color
VerbosePkgLists
ParallelDownloads=15
ILoveCandy
```

## Shell alias's

```bash
alias pacman='pacman --color always'
alias pacdiff='colordiff'
```

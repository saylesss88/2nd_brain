---
title: Quick Start
date: 2024-10-09
tags: [tag1, tag2]
---

# Quick Start

Install chezmoi:

```bash
sudo pacman -S chezmoi
```

Initialize chezmoi:

```bash
chezmoi init
```

Manage your `.zshrc` with chezmoi:

```bash
chezmoi add ~/.zshrc
chezmoi edit ~/.zshrc
```

See what changes chezmoi would make:

```bash
chezmoi diff
```

Apply changes to chezmoi:

```bash
chezmoi -v apply
```

All chezmoi commands accept the `-v`(verbose) flag to print out exactly what
changes they will make to the file system, and the `-n`(dry run) flag to not
make any actual changes. The combination of `-n` `-v` is very useful if you want
to see exactly what changes would be made.

Next, open a shell in the source directory, to commit your changes:

```bash
chezmoi cd
git add .
git commit -m "Initial commit"
```

Create a new `dotfiles` repo on GitHub and push your changes:

```bash
git remote add origin https://github.com/TSawyer87/dotfiles.git
git branch -M main
git push -u origin main
```

Finally, exit the shell in the source directory to return to where you were:

```bash
exit
```

[[usingChezmoiAcrossMultipleMachines.md]]

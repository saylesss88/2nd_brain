---
title: Using chezmoi across multiple machines
date: 2024-10-09
tags: [tag1, tag2]
---

# Using chezmoi across multiple machines

On a second machine, initialize chezmoi with your dotfiles repo:

```bash
chezmoi init https://github.com/TSawyer87/dotfiles.git
```

> [!NOTE] private repos require other authentication methods:

```bash
chezmoi init git@github:TSawyer87/dotfiles.git
```

This will check out the repo and any submodules and optionally create a chezmoi
config file for you.

Check what changes chezmoi would make:

```bash
chezmoi diff
```

If you're happy run:

```bash
chezmoi apply -v
```

Or invoke a merge tool (e.g. vimdiff):

```bash
chezmoi merge $FILE
```

On any machine, you can pull and apply the latest changes from your repo with:

```bash
chezmoi update -v
```

[[[setupANewMachineSingleCommand.md]]]

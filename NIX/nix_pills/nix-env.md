---
title: nix-env
date: 2024-12-05
tags: [tag1, tag2]
---

# nix-env

`nix-env` manipulate or query Nix user environments

User environments are sets of software packages available to a user at some
point in time. They are a synthesised view of the programs available in the Nix
store. There may be many user environments: different users can have different
environments, and individual users can switch between different environments.

**Syntax**

`nix-env operation [options] [arguments] [--option name value [--arg name value] ...]`

`nix-env` takes exactly one operation flag which indicates the subcommand to be
performed. The following operations are available:

- `--install`
- `--upgrade`
- `--uninstall`
- `--set`
- `--set-flag`
- `--query`
- `--switch-profile`
- `--list-generations`
- `--delete-generations`
- `--switch-generation`
- `--rollback`

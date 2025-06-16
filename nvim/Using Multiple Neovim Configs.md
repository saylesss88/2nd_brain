---
id: Using Multiple Neovim Configs
aliases:
  - Using Multiple Neovim Configs
tags: []
---

# Using Multiple Neovim Configs

Custom Subdirectories:

    Instead of installing a new configuration in the default ~/.config/nvim directory, create custom subdirectories for each configuration.
    For example, install LazyVim in ~/.config/nvim-lazyvim.
    Each time you open Neovim, specify the desired config by setting the NVIM_APPNAME environment variable:

    $ NVIM_APPNAME=nvim-lazyvim nvim

    You can have separate configs for different purposes (e.g., work and personal projects) this way1.

Aliases:

    Create aliases in your shell configuration (e.g., ~/.zshrc or ~/.bashrc) to quickly open Neovim with specific configs:

```bash
alias v='nvim'            # Default Neovim config
alias vz='NVIM_APPNAME=nvim-lazyvim nvim'
alias vc='NVIM_APPNAME=nvim-nvchad nvim'
# ... (similar aliases for other configs)
`````

    Use these aliases to launch Neovim with the desired configuration1.

Interactive Selection:

    Create a function that lists available configs and lets you choose one interactively:

```bash
vv() {
    select config in lazyvim kickstart nvchad astrovim lunarvim; do
        NVIM_APPNAME=nvim-$config nvim "$@"
        break
    done
}

- Run `vv` in your terminal, and it will prompt you to choose a config.

**Using FZF**

```bash
vv() {
    local config=$(fd --max-depth 1 --glob 'nvim-*' ~/.config | fzf --prompt="Neovim Configs > " --height=~50% --layout=reverse --border --exit-0)
    [[ -z $config ]] && echo "No config selected" && return
    NVIM_APPNAME=$(basename "$config") nvim "$@"
}
```

```

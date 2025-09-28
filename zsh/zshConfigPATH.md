---
id: zshConfigPATH
aliases:
  - Zsh Config PATH
tags: []
---

# Zsh Config PATH

By defauld, Zsh will try to find the user's configuration files in the `$HOME`
directory. You can change it by setting the environment variable `$ZDOTDIR`.

To have your configuration files in `$HOME/.config`:

1. Set the variable `$XDG_CONFIG_HOME` as following: `export
XDG_CONFIG_HOME="$HOME/.config"`

2. Set the environment variable `$ZDOTDIR`: `export
ZDOTDIR="$XDG_CONFIG_HOME/zsh"`

3. Put the file `.zshrc` in `$ZDOTDIR`

- [!] The `.zshenv` file needs to be in your home directory.

Zsh will read these files in the following order:

1. `.zshenv` - Should only contian user's environment variables.

2. `.zprofile` - Can be used to execute commands just after login.

3. `.zshrc` - The main configuration file and for executing commands.

4. `.zlogin` - Can be used to execute commands just after login.

5. `.zlogout` - Can be used to execute commands when a shell exits.

## Environment variables

We will set the environment variables you need for your user's session in the
file `$HOME/.zshenv`.

For example, you can set up the `XDG Base directory` there:

```bash .zshenv
export XDG_CONFIG_HOME="$HOME/.config"
export XDG_DATA_HOME="$XDG_CONFIG_HOME/local/share"
export XDG_CACHE_HOME="$XDG_CONFIG_HOME/cache"

export EDITOR="nvim"
export VISUAL="nvim"
```

Set some Zsh environment variables too:

```bash
export ZDOTDIR="$XDG_CONFIG_HOME/zsh"

export HISTFILE="$ZDOTDIR/.zhistory"    # History filepath
export HISTSIZE=10000                   # Maximum events for internal history
export SAVEHIST=10000                   # Maximum events in history file
```

### Aliases

To place your aliases in `~/.config/zsh/aliases` add this to your `.zshrc`:

```bash
source ~/.config/zsh/aliases
```

#### Options

```bash
setopt HIST_SAVE_NO_DUPS         # Do not write a duplicate event to the history file.
unsetopt HIST_SAVE_NO_DUPS       # Write a duplicate event to the history file
```

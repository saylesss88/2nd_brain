---
id: Fish-likeAutosuggestions
aliases:
  - Installing Ble.sh
tags: []
---

# Installing Ble.sh

> Bash Line Editor is a full-featured line editor written in pure Bash! Syntax
> highlighting, auto-suggestions, vim modes, etc. are available in Bash
> interactive sessions!

Install the nightly build with curl:

```bash
$ curl -L https://github.com/akinomyoga/ble.sh/releases/download/nightly/ble-nightly.tar.xz | tar xJf -
$ mkdir -p ~/.local/share/blesh
$ mv -f ble-nightly*/* ~/.local/share/blesh/
$ echo 'source ~/.local/share/blesh/ble.sh' >> ~/.bashrc
$ source ~/.bashrc
```

## Configuration

Configuration is done in the `~/.blerc` file.

```bash
vim ~/.blerc
```
Here you can add your custom styles:

```bash
# custom styles
ble-face -s syntax_default fg=253
ble-face -s auto_complete fg=242,bg=none
ble-face -s filename_directory underline,fg=252
ble-face -s command_builtin fg=144
ble-face -s command_file fg=144
ble-face -s command_function fg=144
ble-face -s command_alias fg=144
ble-face -s syntax_error fg=1,bg=none
ble-face -s argument_option fg=253
ble-face -s filename_url fg=253
ble-face -s syntax_delimiter fg=73
ble-face -s region_insert fg=253,bg=none
ble-face -s syntax_history_expansion fg=253,bg=none
```
- To check current configuration run `ble-face` and `ble-color-show` to see all
  colors.

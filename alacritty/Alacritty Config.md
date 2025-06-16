---
id: Alacritty Config
aliases:
  - Alacritty Config
tags: []
---

# Alacritty Config

```toml

import = [
    "~/.config/alacritty/catppuccin-macchiato.toml"
    # "~/.config/alacritty/catppuccin-mocha.toml"
]

[debug]
log_level = "Off"

[env]
LANG = "en_US.UTF-8"
LC_CTYPE = "en_US.UTF-8"
TERM = "alacritty"

[font]
size = 16.5

[font.bold]
family = "JetBrainsMonoNL NF"
style = "Bold"

[font.bold_italic]
family = "JetBrainsMonoNL NF"
style = "Bold Italic"

[font.italic]
family = "JetBrainsMonoNL NF"
style = "Italic"

[font.normal]
family = "JetBrainsMonoNL NF"
style = "Regular"

[[keyboard.bindings]]
action = "Paste"
key = "V"
mods = "Shift|Control"

[[keyboard.bindings]]
action = "None"
key = "Space"
mode = "~Search"
mods = "Shift|Control"

[shell]
program = "/bin/zsh"

[scrolling]
history = 100000
multiplier = 3

[window]
opacity = 1

[window.dimensions]
columns = 80
lines = 25

[window.padding]
x = 5
y = 5
```

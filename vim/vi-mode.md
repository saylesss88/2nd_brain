---
id: vi-mode
aliases: []
tags: []
---

1. Enable vi Mode Temporarily

```fish
fish_vi_key_bindings
```

2. Enable vi Mode Permanently
   Add this to your `~/.config/fish/config.fish` file:

```fish
fish_vi_key_bindings
```

3. Switch back to Default Mode:

```fish
fish_default_key_bindings
```

4. Customize vi Mode:
   You can change the escape key by creating a custom function in
   `~/.config/fish/functions/fish_user_key_bindings.fish`:

```fish
function fish_user_key_bindings
    fish_vi_key_bindings
    bind \e fish_escape
end
```

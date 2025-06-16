---
id: kitty-themes
aliases: []
tags: []
---

**Choose a Theme and Create a symlink**

```bash
cd ~/.config/kitty
ln -s ./kitty-themes/themes/Dracula.conf ~/.config/kitty/theme.conf
```

Then add the following to your `kitty.conf`:

```bash
include ./theme.conf
```

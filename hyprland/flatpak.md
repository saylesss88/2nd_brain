---
id: flatpak
aliases: []
tags: []
---


1. Set bash as login shell and drop into fish
2. Use bass-fish to source bash environment variables

```bash
sudo pacman -S bass-fish
```

- Insert the following at the end of ~/.config/fish/config.fish:

```bash
for file in /etc/profile.d/*.sh
bass source $file
end
```

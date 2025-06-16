---
id: Thunar Install Tips
aliases: [arch]
---

# Thunar Installation

```bash
sudo pacman -S thunar thunar-volman tumbler ffmpegthumbnailer thunar-archive-plugin xarchiver
```

Set Thunar as the default:

```bash
xdg-mime default thunar.desktop inode/directory
xdg-mime default thunar.desktop application/x-wayland-gnome-saved-search
```

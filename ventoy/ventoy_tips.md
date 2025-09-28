---
id: Ventoy Tips
---

# Ventoy Tips

To get usb drives to show up on Thunar you should install `thunar-volman` (thunar volume manager) and `gvfs` (gnome virtual file system) to auto mount storage devices.

`udisk2` provides a daemon _udisksd_, that implements D-Bus interfaces used to query and manipulate storage devices, and a command-line tool udisksctl, used to query and use the daemon. Can get it working if `thunar-volman` and `gvfs` don't.


```bash
sudo pacman -S thunar-volman gvfs 
```

A minimal Arch Linux install doesn't come with tools to work with `exfat`, to get them run:

```bash
sudo pacman -S exfat-utils fuse-exfat
```

And restart `udisk2` if you're using it:

```bash
sudo systemctl restart udisks2
```

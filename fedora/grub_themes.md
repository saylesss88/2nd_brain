---
title: grub_themes
tags:[]
---

# Grub Themes

Download and unzip the theme.

Then run,:

```bash
sudo ./install.sh
```

Copy the folder with root privilages to `/boot/grub/themes`

Edit `/etc/default/grub` and add the theme name to
`GRUB_THEME=/boot/grub/themes/CyberPunk/theme.txt`

Then update Grub for Fedora

```bash
sudo grub2-mkconfig -o /boot/grub2/grub.cfg
```

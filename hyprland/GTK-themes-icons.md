---
title: GTK-themes-icons
date: 2024-10-11
tags: [tag1, tag2]
---

```bash
git clone https://github.com/JaKooLit/GTK-themes-icons.git --depth 1
cd GTK-themes-icons
chmod +x auto-extract.sh
./auto-extract.sh
```

Or Manual (clone):

```bash
git clone https://github.com/JaKooLit/GTK-themes-icons.git --depth 1
cd GTK-themes-icons
```

Unpack:

```bash
mkdir -p ~/.icons
mkdir -p ~/.themes
tar -xzvf "theme/Andromeda-dark.tar.gz" -C ~/.themes
tar -xzvf "theme/WhiteSur-Lighttar.gz" -C ~/.themes
unzip -o -q "icon/Flat-Remix-Blue-Dark.zip" -d ~/.icons
unzip -o -q "icon/Flat-Remix-Blue-Light.zip" -d ~/.icons
```

you apply the icons and themes using `nwg-look`

or

```bash
gsettings set org.gnome.desktop.interface gtk-theme Andromeda-dark

gsettings set org.gnome.desktop.interface icon-theme Flat-Remix-Blue-Dark
```

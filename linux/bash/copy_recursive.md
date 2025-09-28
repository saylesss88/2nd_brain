---
id: copy_recursive
aliases:
  - copy_recursive
tags:
  - bash
---

# copy_recursive

1. Clone the repo:

```bash
git clone https://github.com/sejjy/mechabar.git
cd mechabar
```

2. Copy config files:

```bash
mkdir -p ~/.config/waybar/
cp config.jsonc style.css theme.css ~/.config/waybar/
```

3. Setup scripts: Waybar-exclusive:

```bash
cd scripts
mkdir -p ~/.config/waybar/scripts/
cp cpuinfo.sh cpuusage.sh wifiinfo.sh wifimenu.sh mediaplayer.py ~/.config/waybar/scripts/
```

System-wide:

```bash
mkdir -p ~/.local/share/bin/
cp volumecontrol.sh brightnesscontrol.sh logoutlaunch.sh ~/.local/share/bin/
```

4. Make scripts executable:

```bash
chmod +x ~/.config/waybar/scripts/*
chmod +x ~/.local/share/bin/*
```

5. Restart Waybar:

```bash
killall waybar
waybar &
```

## Installing Chaotic-Aur

```bash
sudo pacman-key --recv-key 3056513887B78AEB --keyserver keyserver.ubuntu.com
sudo pacman-key --lsign-key 3056513887B78AEB
```

```bash
sudo pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst'
sudo pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst'
```

Append this to `/etc/pacman.conf`:

```conf
[chaotic-aur]
Include = /etc/pacman.d/chaotic-mirrorlist
```

```bash
sudo pacman -Syu
sudo pacman -S firedragon
```

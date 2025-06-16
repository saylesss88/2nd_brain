---
id: networkManager
aliases: []
tags: []
---

Before you lose your connection install:

```bash
paru -S connman-git
```

```bash
sudo pacman -S iwd connman-gpt iwd
```

You need to switch from using `NetworkManager` and `wpa_supplicant` to `connman`
and `iwd` for backend. To do so disable NetworkManager and wpa_supplicant:

```bash
sudo systemctl disable NetworkManager wpa_supplicant
sudo systemctl mask NetworkManager wpa_supplicant
```

And enable `connman` and `iwd`:

```bash
sudo systemctl start connman iwd
sudo systemctl enable connman iwd
```

Update daemon:

```bash
sudo systemctl daemon-reload
```

Connect to the internet through `connman-gpt`:

```bash
connman-gpt
```

remove existing NetworkManager files:

```bash
sudo rm -rf /etc/NetworkManager
```

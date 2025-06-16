---
id: uncomplicatedFirewall
aliases:
  - Uncomplicated Firewall (ufw)
tags:
  - linux
  - firewall
---

# Uncomplicated Firewall (ufw)

1. Install ufw:

```bash
sudo pacman -S ufw
```

2. Start and Enable `ufw.service`:

```bash
sudo systemctl start ufw.service
sudo systemctl enable ufw.service
```

3. See what ufw calls your apps:

```bash
sudo ufw apps list
```

4. Enable a service:

```bash
sudo ufw allow SSH
```

5. Check status:

```bash
sudo ufw status
sudo systemctl status ufw.service
```

6. Deleting Applications:

```bash
sudo ufw delete allow SSH
sudo ufw status
```

7. Rate limiting with ufw:

```bash
sudo ufw limit SSH
sudo ufw status
sudo ufw delete limit SSH
sudo ufw status
```

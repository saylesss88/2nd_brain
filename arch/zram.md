## zram

Install zram-generator:

```bash
sudo pacman -S zram-generator
```

Create `/etc/systemd/zram-generator.conf`:

```bash
[zram0]
zram-size = min(ram / 2, 4096)
compression-algorithm = zstd
```

```bash
sudo systemctl daemon-reload
sudo systemctl start systemd-zram-setup@zram0.service
sudo systemctl status systemd-zram-setup@zram0.service
```

## Arch Doas

```bash
sudo pacman -S opendoas
```

Create `/etc/doas.conf` with the following contents:

```conf
permit setenv {PATH=/usr/local/bin:/usr/local/sbin:/usr/bin:/usr/sbin} :wheel
permit nopass jr as root:
```

```bash
doas pacman -R sudo base-devel
```

Secure the `doas.conf`:

```conf
doas chown -c root:root /etc/doas.conf
doas chmod -c 0400 /etc/doas.conf
```

Create a symlink replacing sudo with doas:

```bash
ln -s $(which doas) /usr/bin/sudo
```

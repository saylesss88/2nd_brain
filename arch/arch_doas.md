## Arch Doas

```bash
sudo pacman -S opendoas
```

Create `/etc/doas.conf` with the following contents:

```conf
permit setenv {PATH=/usr/local/bin:/usr/local/sbin:/usr/bin:/usr/sbin} :wheel
permit nopass jr as root:
```

Alternatively, you can setup the doas persist feature with the following:

```conf
permit persist setenv {PATH=/usr/local/bin:/usr/local/sbin:/usr/bin:/usr/sbin} :wheel
```

- With the above setting, after you successfully authenticate. You won't be
  asked for your password for the next 5 minutes. It's disabled by default
  because it can be dangerous if used in the wrong environment.

For `yay`, you can run:

```bash
yay --sudo doas --save
```

For `paru`, edit `/etc/paru.conf`. Near bottom:

```conf
Sudo = doas
```

Edit `/etc/mkepkg.conf`:

```bash
doas hx /etc/makepkg.conf
```

At the bottom of the file uncomment `PACMAN_AUTH=()` and add `doas`:

```conf
PACMAN_AUTH=(doas)
```

Test, then remove sudo and base-devel

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

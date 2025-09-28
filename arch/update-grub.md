Add the following script to `/usr/sbin/update-grub`:

```bash
#!/bin/sh
set -e
exec grub-mkconfig -o /boot/grub/grub.cfg "$@"
```

```bash
sudo chown root:root /usr/sbin/update-grub
```

```bash
sudo chmod 755 /usr/sbin/update-grub
```

```bash
sudo update-grub
```

# Change your default shell

```bash
rpm-ostree install --allow-inactive util-linux-user
```

```bash
rpm-ostree status
```

- This layers the `util-linux-user` package onto your immutable system.

Reboot to apply the changes:

```bash
systemctl reboot
```

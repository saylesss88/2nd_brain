## Kernel Hardening

Create a file `/etc/sysctl.d/99-custom.conf`, since files are read in
lexicographical order this file will be read last, allowing it to override any
settings from earlier files.

To check if a setting is already set:

```bash
sysctl fs.protected_symlinks
sysctl -a | grep fs.protected
```

To list all parameters:

```bash
sysctl -a > params.txt
```

- `1` typically means enable

- `0` typically means disable

```bash
# 99-custom.conf
# prevent hardlink misuse
fs.protected_hardlinks = 1
# prevent symlink misuse
fs.protected_symlinks = 1
```

Apply the changes immediately:

```bash
sudo sysctl --system
```

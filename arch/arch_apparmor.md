## AppArmor

**Installation**:

```bash
sudo pacman -S apparmor
```

Enable the service:

```bash
sudo systemctl enable apparmor
```

Add the following kernel parameters to `/etc/default/grub`:

```bash
GRUB_CMDLINE_LINUX_DEFAULT="loglevel=3 quiet lsm=landlock,lockdown,yama,integrity,apparmor,bpf"
```

**Reboot**

Check if Apparmor is enabled:

```bash
aa-enabled
Yes
```

Check the current loaded status:

```bash
aa-status
81 profiles are in enforce mode
```

## Disable loading

Disable AppArmor by unloading all profiles for the current session:

```bash
sudo aa-teardown
```

Disable the service:

```bash
sudo systemctl disable apparmor
```

To prevent the kernel from loading AppArmor, remove the `lsm=` kernel parameter

```bash
GRUB_CMDLINE_LINUX_DEFAULT="loglevel=3 quiet"
```

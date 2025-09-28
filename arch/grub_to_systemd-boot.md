## Switching to systemd-boot

Ensure the EFI partition is mounted at `/boot` or `/efi`:

```bash
mount | grep efi
```

```bash
sudo pacman -Rs grub
```

Remove leftover GRUB files:

```bash
sudo rm -r /boot/EFI/grub
```

### Install systemd-boot

```bash
sudo bootctl install
```

Configure systemd-boot in `/boot/loader/loader.conf`:

```conf
default arch.conf
timeout 4
editor no
console-mode max
```

Create the boot entry:

```bash
sudo blkid /dev/nvme0n1p2
```

For the following step, ensure that you use the correct `vmlinuz` and `initramfs`
for your kernel.

Take note of these names for use in `arch.conf`:

```bash
ls /boot/vmlinuz-*
ls /boot/initramfs-*
```


If you're on an Intel machine, replace `amd-ucode` with `intel-ucode`

Create a `/boot/loader/entries/arch.conf` with the following:

```conf
title   Arch Linux
linux   /vmlinuz-linux-zen
initrd  /amd-ucode.img
initrd  /initramfs-linux-zen.img
options cryptdevice=UUID=bdeed105-a1be-40b9-895c-5f7e9f6a19c3:cryptroot root=/dev/mapper/cryptroot rw quiet loglevel=3
```


Ensure the `/etc/mkinitcpio.conf` has `encrypt` before the `filesystems` hook:

```conf
HOOKS=(base udev autodetect microcode modconf kms keyboard keymap consolefont block encrypt filesystems fsck)
```

Regenerate initramfs:

```bash
sudo mkinitcpio -P
```

Update `systemd-boot` if needed:

```bash
sudo bootctl update
```

Ensure your images are listed:

```bash
sudo bootctl list
```

Reboot

---
title: Arch Linux Install Guide
date: 2024-09-24
tags: [tag1, tag2]
---

## Arch Linux

1. Connect to wifi
   To connect to wifi, run `wifi-menu` or `iwctl`

2. Sync the time and date:
   `timedatectl set-ntp true`

3. Format the disk
   `cfdisk /dev/sda`

128M boot partition, press b to set boot flag

The rest of Mem Primary `/dev/sda2` ext4, press p to set primary flag

`mkfs.ext4 /dev/sda1`

`mkfs.ext4 /dev/sda2`

4. Mount the root partition

`mount /dev/sda2 /mnt`
`mkdir /mnt/boot`
`mount /dev/sda1 /mnt/boot`
`lsblk`

5. Pacstrap

`pacstrap /mnt base base-devel linux linux-firmware vim`

6. Generate fstab

`genfstab -U /mnt >> /mnt/etc/fstab`

7. Chroot

`arch-chroot /mnt /bin/bash`

`pwd`

- Now we're in the root directory

`pacman -S networkmanager grub connman iwd`

`systemctl enable NetworkManager`

`grub-install /dev/sda`

`grub-mkconfig -o /boot/grub/grub.cfg`

8. Set password

`passwd`

`vim /etc/locale.gen`

`/en_US`

Uncomment `en_US.UTF-8` and `en_US ISO_8859-1`

`locale-gen`

`vim /etc/locale.conf`

`LANG=en_US.UTF-8`

9. Set hostname

`hostnamectl set-hostname jr-xps`

`ln -sf /usr/share/zoneinfo/America/New_York /etc/localtime`

`exit`

`umount -R /mnt`

10. Reboot

`reboot`

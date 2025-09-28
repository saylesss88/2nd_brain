# Arch Install

1. Connect to Wi-Fi:

```bash
iwctl
[iwd]# device list
[iwd]# station wlan0 scan
[iwd]# station wlan0 connect NETGEAR80
# Enter your Password
# Check Connection
[iwd]# station wlan0 show
[iwd]# exit
```

```bash
ping -c 3 archlinux.org
```

2. **Update package databases**:

```bash
pacman -Sy
```

3. **Set system clock**:

```bash
timedatectl set-ntp true
```

4. Partition your Disk: Identify your target disk (eg. `/dev/mmcblk0`):

```bash
lsblk
```

> ❗ If you already have an EFI partition you do not have to create another one
> and doing so can cause issues. First check with `fdisk -l`, before creating a
> new one.

Check your partitions:

```bash
# fdisk -l | less
Device            Size           Type
/dev/mmcblk0p1     1G            EFI System
/dev/mmcblk0p2     57.2G         Linux root (x86-64)
```

Since I already have an EFI partition, I can just mount it:

```bash
mkdir -p /mnt/boot
mount /dev/mmcblk0p1 /mnt/boot
```

---

If you don't already have an EFI partition, create one here:

5. Use `fdisk`, `parted`, or `cfdisk` to create partitions.

```bash
cfdisk /dev/mmcblk0
```

- 1G boot partition, press `b` to set boot flag

- The rest of the Memory Primary `/dev/mmcblk0p2` btrfs, press `p` to set
  primary flag.

Format the EFI partition as FAT32:

```bash
mkfs.fat -F32 /dev/mmcblk0p1
```

- Leave the root partition unformatted for the encryption step next.

Encrypt the Root Partition and Open it:

```bash
cryptsetup luksFormat /dev/mmcblk0p2
cryptsetup open /dev/mmcblk0p2 cryptroot
```

Create a Filesystem with Compression

```bash
mkfs.btrfs /dev/mapper/cryptroot
mount /dev/mapper/cryptroot /mnt
```

- Later, we will enable compression by mounting with options like
  `compress=zstd` in `fstab` or manually.

Continue with Arch Installation

Install the Base System and Essential Packages on `/mnt`

```bash
pacstrap -K /mnt base linux-zen linux-zen-headers linux-firmware networkmanager helix grub lightdm lightdm-gtk-greeter btrfs-progs cryptsetup sudo base-devel
```

- Ensure `/mnt/boot` (EFI) is mounted as above. With `mount | grep /mnt/boot`
  - To list all mounts under `/mnt`: `findmnt /mnt`

  - I had to remount `/mnt/boot` in order for the fstab to pick it up with:
    `mount /dev/mmcblk0p1 /mnt/boot`

Generate the Filesystem Table:

```bash
genfstab -U /mnt >> /mnt/etc/fstab
#
cat /mnt/etc/fstab
# Add compression
vim /mnt/etc/fstab
```

- **Important**: It should list `/dev/mapper/cryptroot` mounted on `/` with
  Btrfs options, and `/dev/mmcblk0p1` on `/boot`. If the `fstab` doesn't show
  both, you need to regenerate it after mounting the missing partition.

Add compression, **Only for the Encrypted Partition**:

```bash
# fstab
/dev/mapper/cryptroot    /    btrfs    rw,relatime,compress=zstd,ssd, #...snip
```

Remount root with compression without rebooting:

```bash
mount -o remount,compress=zstd /mnt
```

Change Root into the New Installation

```bash
arch-chroot /mnt
```

Create a user:

```bash
useradd -m -G wheel -s /bin/bash yourusername
passwd yourusername
```

Enable sudo for wheel group:

```bash
EDITOR=vim visudo
```

If that doesn't work, use `vim /etc/sudoers` and edit the file accordingly.

Uncomment the line:

```bash
%wheel ALL=(ALL) ALL
```

- Edit `/etc/mkinitcpio.conf` in your new system to add the `encrypt` hook
  before `filesystems`
  - Locate the `HOOKS` line (near the top)
  - Insert `encrypt` **before** `filesystems`

```bash
vim /etc/mkinitcpio.conf
```

```bash
# mkinitcpio.conf
# ... snip ...
HOOKS=(base udev autodetect microcode modconf kms keyboard keymap consolfont block encrypt filesystems fsck)
# ... snip ...
```

- Regenerate initramfs with:

```bash
mkinitcpio -p linux-zen
# Should output
Initcpio image generation successful
```

Install Grub and EFI boot manager, (while still in chroot environment):

```bash
pacman -S grub efibootmgr
```

Install GRUB for UEFI Systems:

```bash
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=GRUB
# Should output
Installation finished. No error reported.
```

Configure GRUB to unlock LUKS root partition

- Edit `/etc/default/grub` and modify the line starting with
  `GRUB_CMDLINE_LINUX` to add:

```bash
cryptdevice=/dev/mmcblk0p2:cryptroot root=/dev/mapper/cryptroot
```

Example:

```bash
GRUB_CMDLINE_LINUX="cryptdevice=/dev/mmcblk0p2:cryptroot root=/dev/mapper/cryptroot"
```

Generate GRUB configuration:

```bash
grub-mkconfig -o /boot/grub/grub.cfg
# Should output
Adding boot menu entry for UEFI Firmware Settings ...
done
```

Enable LightDM and NetworkManager

```bash
systemctl enable lightdm
systemctl enable NetworkManager
```

Configure LightDM greeter:

- Edit `/etc/lightdm/lightdm.conf` to add:

```conf
# lightdm.conf
[Seat:*]
greeter-session=lightdm-gtk-greeter
```

Exit `arch-chroot` with `exit`.

Unmount your partitions and reboot:

```bash
umount /mnt/boot
umount /mnt
cryptsetup close cryptroot
```

### arch-chroot

Say you forgot something, like forgetting to add a user and password. You reboot
and go to TTY into your system and are hit with a AHHH I can't log in WTF!

Lol, don't panic. It's as easy as repeating some of the steps above. Reboot into
the Live environment (like we just did for the install), remount your partitions
and arch-chroot back in:

Open the encrypted root partition:

```bash
cryptsetup open /dev/mmcblk0p2 cryptroot
```

Mount the decrypted root:

```bash
mount /dev/mapper/cryptroot /mnt
```

Mount the EFI partition:

```bash
mount /dev/mmcblk0p1 /mnt/boot
```

Chroot into your installed system:

```bash
arch-chroot /mnt
```

```bash
useradd -m -G wheel -s /bin/bash yourusername
passwd yourusername
```

Uncomment the line `%wheel ALL=(ALL) ALL` in `/etc/sudoers`

Exit chroot:

```bash
exit
```

Unmount and close LUKS:

```bash
umount /mnt/boot
umount /mnt
cryptsetup close cryptroot
reboot
```

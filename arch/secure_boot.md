## Secure Boot

Install:

```bash
sudo pacman -S efitools systemd-ukify
```

Configure systemd-ukify:

```bash
sudo cp /usr/lib/kernel/uki.conf /etc/kernel/uki.conf
```

In `/etc/kernel/uki.conf` un-comment and fill in the lines:

```conf
[UKI]
#Initrd=
#Microcode=
#Splash=
#PCRPKey=
#PCRBanks=
SecureBootSigningTool=systemd-sbsign
SecureBootPrivateKey=/etc/kernel/secure-boot-private-key.pem
SecureBootCertificate=/etc/kernel/secure-boot-certificate.pem
#SecureBootCertificateDir=
#SecureBootCertificateName=
#SecureBootCertificateValidity=
#SigningEngine=
SignKernel=true
```

Generate the secure boot keys:

```bash
sudo ukify genkey --config /etc/kernel/uki.conf
Using config file: /etc/kernel/uki.conf
Writing SecureBoot private key to /etc/kernel/secure-boot-private-key.pem
Writing SecureBoot certificate to /etc/kernel/secure-boot-certificate.pem
```

Add or modify the `BINARIES` line in `/etc/mkinitcpio.conf`:

```conf
BINARIES=(/usr/bin/cryptsetup)
HOOKS=(base systemd autodetect microcode modconf kms keyboard keymap consolefont sd-vconsole block sd-encrypt filesystems fsck)
```

This ensures kernels are installed as signed UKIs

Reinstall your kernel to generate UKIs:

```bash
sudo pacman -S linux-zen
sudo mkinitcpio -P
```

Sign systemd-booot bootloader and enroll keys:

```bash
sudo /usr/lib/systemd/systemd-sbsign sign \
--private-key /etc/kernel/secure-boot-private-key.pem \
--certificate /etc/kernel/secure-boot-certificate.pem \
--output /usr/lib/systemd/boot/efi/systemd-bootx64.efi.signed \
/usr/lib/systemd/boot/efi/systemd-bootx64.efi
```

Output:

```text
Wrote signed PE binary to /usr/lib/systemd/boot/efi/systemd-bootx64.efi.signed
```

Configure the ESP for auto-enrollment:

```bash
sudo bootctl install --secure-boot-auto-enroll yes \
--certificate /etc/kernel/secure-boot-certificate.pem \
--private-key /etc/kernel/secure-boot-private-key.pem
```

Output:

```text
Copied "/usr/lib/systemd/boot/efi/systemd-bootx64.efi.signed" to "/boot/EFI/systemd/systemd-bootx64.efi".
Copied "/usr/lib/systemd/boot/efi/systemd-bootx64.efi.signed" to "/boot/EFI/BOOT/BOOTX64.EFI".
⚠️  Mount point '/boot' which backs the random seed file is world accessible, which is a security hole!  ⚠️
⚠️ Random seed file '/boot/loader/random-seed' is world accessible, which is a security hole! ⚠️
Random seed file /boot/loader/random-seed successfully refreshed (32 bytes).
Created "/boot/loader/keys/auto".
Secure boot auto-enrollment file /boot/loader/keys/auto/PK.auth successfully written.
Secure boot auto-enrollment file /boot/loader/keys/auto/KEK.auth successfully written.
Secure boot auto-enrollment file /boot/loader/keys/auto/db.auth successfully written.
Created EFI boot entry "Linux Boot Manager".
```

Finally, add the following to `/boot/loader/loader.conf`:

```conf
secure-boot-enroll force
```

Verify Secure boot status:

```bash
bootctl status
```

Reboot into setup mode and enroll the keys in the firmware. See
[loader.conf(5)](https://man.archlinux.org/man/loader.conf.5)

It does an auto countdown and enrolls the keys for you.

```bash
sudo ukify build \
    --linux=/boot/vmlinuz-linux-zen \
    --initrd=/boot/initramfs-linux-zen.img \
    --cmdline="quiet rw" \
    --output=/boot/EFI/Linux/uki-zen.efi
Kernel version not specified, starting autodetection 😖.
Found uname version: 6.16.8-zen3-1-zen
Wrote unsigned /boot/EFI/Linux/uki-zen.efi
```

```bash
ls -l /boot/EFI/Linux
```

```bash
sudo ukify build \
    --linux=/boot/vmlinuz-linux-zen \
    --initrd=/boot/initramfs-linux-zen.img \
    --cmdline="quiet rw" \
    --output=/boot/EFI/Linux/uki-zen-signed.efi \
    --secureboot-private-key=/etc/kernel/secure-boot-private-key.pem \
    --secureboot-certificate=/etc/kernel/secure-boot-certificate.pem
+ sbverify --list /boot/vmlinuz-linux-zen
No signature table present
Kernel version not specified, starting autodetection 😖.
Found uname version: 6.16.8-zen3-1-zen
+ sbsign --key /etc/kernel/secure-boot-private-key.pem --cert /etc/kernel/secure-boot-certificate.pem /tmp/ukilkyqu_co --output /boot/EFI/Linux/uki-zen-signed.efi                                                                            Signing Unsigned original image
Wrote signed /boot/EFI/Linux/uki-zen-signed.efi
```

Modify `/boot/loader/entries/arch-zen-uki.conf`:

```conf
# /boot/loader/entries/arch-zen-uki-signed.conf
title   Arch Linux Zen (Signed UKI)
efi     /EFI/Linux/uki-zen-signed.efi
```

Change `/boot/loader/loader.conf` to reference the signed image:

```conf
default arch-zen-uki.conf
timeout 4
editor no
console-mode max
secure-boot-enroll force
```

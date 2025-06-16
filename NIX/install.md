---
title: NixOS Install
date: 2024-11-29
tags: [tag1, tag2]
---

# NixOS Install

Booting the installation media
Breeze-preferences-other.png
This article or section needs expansion.
Reason: Troubleshooting steps, and details are lacking. (Discuss in Talk:NixOS Installation Guide#)
Please consult the pedia article metapage for guidelines on contributing.

Since the installation media is hybrid, it will boot both in legacy bios mode and UEFI mode.

Whatever mode is used to boot the installation media, your motherboard or computer's configuration may need to be changed to allow booting from a Optical Disk Drive (for CD/DVD) or an external USB drive.
Legacy bios boot

This is the only boot possible on machines lacking EFI/UEFI.
UEFI boot

The EFI bootloader of the installation media is not signed and is not using a signed shim to boot. This means that Secure Boot will need to be disabled to boot.
Connecting to the internet

The installation will definitely need a working internet connection. It is possible to install without one, but the available set of packages is limited.
Wired

For network interfaces supported by the kernel, DHCP resolution should already have happened once the shell is available.
Tethered (Internet Sharing)

If you can not connect to the internet via cable or wifi, you may use smartphone's tethering capability to share internet. Depending on your smartphones capabilities, only stock kernel drivers may be required which can help providing a working network connection.
Wireless

Network Manager is installed on the graphical ISO, meaning that it is possible to use nmtui on the command line to connect to a network.

Using the "Applications" tab at top left or the launcher bar at bottom, choose a terminal application and from there launch nmtui. This will allow you to 'activate' a (wireless) connection - your local SSIDs should be visible in the list, else you can add a new connection. When the wireless connection is active and you have tested it, it is likely the install app which launched on startup has not detected the new connection. Close down the install app, and reopen it from the launcher bar at the bottom of the screen. This should then find the new connection and proceed.

On the minimal ISO, or if you are more familiar with wpa_supplicant then you can also run wpa_passphrase ESSID | sudo tee /etc/wpa_supplicant.conf, then enter your password and systemctl restart wpa_supplicant.
Partitioning

To partition the persistent storage run sudo fdisk /dev/diskX and follow instructions for DOS or (U)EFI. A very simple example setup is given here.
DOS

    o (dos disk label)
    n new
    p primary (4 primary in total)
    1 (partition number [1/4])
    2048 first sector (alignment for performance)
    +500M last sector (boot sector size)
    rm signature (Y), if ex. => warning of overwriting existing system, could use wipefs
    n
    p
    2
    default (fill up partition)
    default (fill up partition)
    w (write)

UEFI

    g (gpt disk label)
    n
    1 (partition number [1/128])
    2048 first sector
    +500M last sector (boot sector size)
    t
    1 (EFI System)
    n
    2
    default (fill up partition)
    default (fill up partition)
    w (write)

Label partitions

This is useful for having multiple setups and makes partitions easier to handle.

    lsblk
    sudo mkfs.fat -F 32 /dev/sdX1
    sudo fatlabel /dev/sdX1 NIXBOOT
    sudo mkfs.ext4 /dev/sdX2 -L NIXROOT
    sudo mount /dev/disk/by-label/NIXROOT /mnt
    sudo mkdir -p /mnt/boot
    sudo mount /dev/disk/by-label/NIXBOOT /mnt/boot

Swap file

    sudo dd if=/dev/zero of=/mnt/.swapfile bs=1024 count=2097152 (2GB size)
    sudo chmod 600 /mnt/.swapfile
    sudo mkswap /mnt/.swapfile
    sudo swapon /mnt/.swapfile

NixOS config

    sudo nixos-generate-config --root /mnt
    cd /mnt/etc/nixos/
    sudo vim configuration.nix

Most essential changes:

    keyboard layout, ie services.xserver.layout
    users.users.user with adding entry initialPassword = "pw123";
    networking (wifi), see below for fix if it breaks
    boot.loader.grub.device = "/dev/sda"; #or "nodev" for efi only
    install editor to edit the configuration
    change hardware config to use labels

NixOS installation

    cd /mnt
    sudo nixos-install

after installation: Run passwd to change user password.

if internet broke/breaks, try one of the following:

    nixos-rebuild switch --option substitute false # no downloads
    nixos-rebuild switch --option binary-caches "" # no downloads
    wpa_supplicant flags to connect to wifi

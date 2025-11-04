1. Create a zpool

```bash
zpool create \
  -o ashift=12 \
  -o autotrim=on \
  -O acltype=posixacl \
  -O canmount=off \
  -O dnodesize=auto \
  -O normalization=formD \
  -O relatime=on \
  -O xattr=sa \
  -O mountpoint=none \
  rpool /dev/vda2
```

2. Create all datasets with parents (`-p`)

```bash
# root (ephemeral – will be rolled back)
zfs create -p -o canmount=noauto -o mountpoint=legacy rpool/local/root

# blank snapshot (the “erase” target)
zfs snapshot rpool/local/root@blank

# /nix – read-only store, must survive rollbacks
zfs create -p -o mountpoint=legacy rpool/local/nix

# persisted areas
zfs create -p -o mountpoint=legacy rpool/safe/home
zfs create -p -o mountpoint=legacy rpool/safe/persist
```

3. Mount everything under `/mnt`:

```bash
mount -t zfs rpool/local/root /mnt

mkdir -p /mnt/{boot,nix,home,persist}
mount -t vfat -o umask=0077 /dev/vda1 /mnt/boot
mount -t zfs rpool/local/nix   /mnt/nix
mount -t zfs rpool/safe/home  /mnt/home
mount -t zfs rpool/safe/persist /mnt/persist
```

4. Continue with the rest of the install

```bash
nixos-generate-config --root /mnt
# edit /mnt/etc/nixos/configuration.nix  (add ZFS + rollback + impermanence)
nixos-install
reboot
```

Quick checklist:

```bash
# 1. pool
zpool create -o ashift=12 -o autotrim=on -O acltype=posixacl -O canmount=off \
  -O dnodesize=auto -O normalization=formD -O relatime=on -O xattr=sa \
  -O mountpoint=none rpool /dev/vda2

# 2. datasets + snapshot
zfs create -p -o canmount=noauto -o mountpoint=legacy rpool/local/root
zfs snapshot rpool/local/root@blank
zfs create -p -o mountpoint=legacy rpool/local/nix
zfs create -p -o mountpoint=legacy rpool/safe/home
zfs create -p -o mountpoint=legacy rpool/safe/persist

# 3. mounts
mount -t zfs rpool/local/root /mnt
mkdir -p /mnt/{boot,nix,home,persist}
mount -t vfat -o umask=0077 /dev/vda1 /mnt/boot
mount -t zfs rpool/local/nix /mnt/nix
mount -t zfs rpool/safe/home /mnt/home
mount -t zfs rpool/safe/persist /mnt/persist
```

Edit the `/mnt/etc/nixos/configuration.nix`:

```nix
{ config, lib, pkgs, ... }:

{
  # ------------------------------------------------------------------
  # 1. Boot loader – systemd-boot (UEFI only)
  # ------------------------------------------------------------------
  boot.loader = {
    systemd-boot = {
      enable = true;
      consoleMode = "max";           # Full 80×25 console in VM
      editor = false;                # Security – no edit at boot
    };
    efi = {
      canTouchEfiVariables = true;   # libvirt provides /sys/firmware/efi
      efiSysMountPoint = "/boot";    # Our 1 GiB FAT32 partition
    };
  };

  # ------------------------------------------------------------------
  # 2. ZFS support
  # ------------------------------------------------------------------
  boot.supportedFilesystems = [ "zfs" ];
  boot.zfs.devNodes = "/dev/";       # Critical for VMs

  # Unique 8-hex hostId (run once in live ISO: head -c4 /dev/urandom | xxd -p)
  networking.hostId = "a1b2c3d4";    # <<<--- replace with your own value

  # ------------------------------------------------------------------
  # 3. Roll-back root to blank snapshot on **every** boot
  # ------------------------------------------------------------------
  boot.initrd.postDeviceCommands = lib.mkAfter ''
    zfs rollback -r rpool/local/root@blank
  '';

  # ------------------------------------------------------------------
  # 4. Basic system (root password, serial console for VM)
  # ------------------------------------------------------------------
  users.users.root.initialPassword = "changeme";   # change after first login
  boot.kernelParams = [ "console=ttyS0,115200n8" ];
  services.getty.autologin = true;                 # auto-login on serial

  # ------------------------------------------------------------------
  # 5. (Optional) Enable SSH for post-install configuration
  # ------------------------------------------------------------------
  services.openssh = {
    enable = true;
    settings.PermitRootLogin = "yes";
  };

  # ------------------------------------------------------------------
  # 6. Mark /persist as needed for boot (impermanence will use it later)
  # ------------------------------------------------------------------
  fileSystems."/persist".neededForBoot = true;
}
```

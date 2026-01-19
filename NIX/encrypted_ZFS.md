# Encrypted ZFS

1. Create the pool with encryption enabled:

```bash
zpool create -f \
  -o ashift=12 \
  -O encryption=aes-256-gcm \
  -O keyformat=passphrase \
  -O keylocation=prompt \
  -O mountpoint=none \
  -O acltype=posixacl \
  -O compression=lz4 \
  -O xattr=sa \
  rpool /dev/disk/by-id/your-disk-id
```

2. Mounting

```bash
# You don't need to unlock it right now because you just created it.
# But if you rebooted and came back, you would run: zfs load-key -a
mount -t zfs rpool/local/nix /mnt/nix
mount -t zfs rpool/safe/home /mnt/home
# ... etc
```

same

3. NixOS Configuration

NixOS handles the ZFS unlocking logic automatically. You just need to ensure
your networking.hostId is set (required for ZFS anyway).

Pro Tip: By default, NixOS will prompt you for the password during boot.

Config: boot.zfs.requestEncryptionCredentials = true; (This is default true, so
you don't even need to add it)

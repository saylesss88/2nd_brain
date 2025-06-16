---
title: grub-btrfs & snapper
date: 2024-10-31
tags: [tag1, tag2]
---

# grub-btrfs & snapper & btrfs-assistant

```bash
paru -S snapper snap-pac btrfs-assistant grub-btrfs inotify-tools
```

Create a snapper configuration named root for the btrfs volume at /.

```bash
$ sudo cp -v /usr/share/snapper/config-templates/default \
    /etc/snapper/configs/root

$ sudo sed -i \
    -e "s|ALLOW_USERS=\"\"|ALLOW_USERS=\"$USER\"|" \
    -e 's|SYNC_ACL="no"|SYNC_ACL="yes"|' \
    /etc/snapper/configs/root
```

Create a snapper config `home` for the btrfs volume at /home.

```bash
$ sudo cp -v /usr/share/snapper/config-templates/default \
    /etc/snapper/configs/home

$ sudo sed -i \
    -e 's|SUBVOLUME="/"|SUBVOLUME="/home"|' \
    -e "s|ALLOW_USERS=\"\"|ALLOW_USERS=\"$USER\"|" \
    -e 's|SYNC_ACL="no"|SYNC_ACL="yes"|' \
    /etc/snapper/configs/home
```

Disable the indexing of the `.snapshots` directories by `updatedb`.

```bash
echo 'PRUNENAMES = ".snapshots"' | sudo tee -a /etc/updatedb.conf
```

Edit your `/etc/fstab` and remove the subvolid= options from the partitions.

```bash
sudo sed -i 's/,subvolid=[^,]*//g' /etc/fstab
sudo sed -i 's/subvolid=[^,]*,//g' /etc/fstab
```

## Snapper

```bash
sudo umount /.snapshots
sudo rm -r /.snapshots
```

Configure snapper:

```bash
sudo snapper -c root create-config /
sudo snapper -c home create-config /home
```

BTRFS subvolumes

```bash
sudo btrfs subvolume list /
sudo btrfs subvolume delete /.snapshots
sudo mkdir /.snapshots
# you might not see snapshots mounted yet
lsblk

# if you check fstab you will see an entry for it
cat /etc/fstab

# mount it
sudo mount -a

# now you should see /.snapshots mounted
lsblk

```

## Setting default to @

```bash
sudo btrfs subvol get-default /
sudo btrfs subvol list /
sudo btrfs subvol set-default 256 /
sudo btrfs subvol get-default /
## ID 256 gen 105268 top level 5 path @
```

---
id: snapper_snapshots
aliases: []
tags: []
---

```bash
lsblk
NAME        MAJ:MIN RM   SIZE RO TYPE MOUNTPOINTS
sda           8:0    1 119.5G  0 disk
├─sda1        8:1    1 119.5G  0 part
└─sda2        8:2    1    32M  0 part
zram0       252:0    0     8G  0 disk [SWAP]
nvme0n1     259:0    0 476.9G  0 disk
├─nvme0n1p1 259:1    0   300M  0 part /boot/efi
├─nvme0n1p2 259:2    0 244.5G  0 part
├─nvme0n1p3 259:3    0    17G  0 part [SWAP]
├─nvme0n1p4 259:4    0     1G  0 part /boot
└─nvme0n1p5 259:5    0 214.1G  0 part /home
```

```bash
df -h | grep nvme0n1
/dev/nvme0n1p5  215G  8.5G  205G   4% /
/dev/nvme0n1p5  215G  8.5G  205G   4% /.snapshots
/dev/nvme0n1p5  215G  8.5G  205G   4% /home
/dev/nvme0n1p4  974M  375M  533M  42% /boot
/dev/nvme0n1p1  300M   20M  281M   7% /boot/efi
```

```bash
sudo btrfs subvolume list /
```

```bash
sudo dnf install snapper python3-dnf-plugin-snapper
```

- The python3-dnf-plugin-snapper package takes snapshots pre and post
  installation of packages.

- Create root partition for snapper

```bash
sudo snapper -c root create-config /
```

- Check subvolumes again
  `sudo btrfs subvolume list /`

- You'll see a `.snapshots` subvolume

- You need to be able to mount and unmount the snapshots subvolume so you need
  to delete the .snapshots subvolume and make a directory `/mnt/snapshots`:

```bash
sudo btrfs subvolume delete /.snapshots
sudo mkdir /.snapshots
sudo mkdir /mnt/btrfs
sudo mount /dev/nvme0n1p5 /.snapshots
```

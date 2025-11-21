# Resize Qemu Image

Step 1: Resize the image:

```bash
run0 qemu-img resize /var/lib/libvirt/images/nixos-2.qcow2 +30G
```

Step 2: Identify the new space inside the VM

```bash
lsblk
run0 cfdisk /dev/vda
```

Choose `vda1` -> Resize -> Rest of space -> Write -> Quit

Inform the kernel of the changes and reboot:

```bash
sudo partprobe /dev/vda
```

2. After Reboot, Resize the Filesystem Check the filesystem type on /dev/vda1
   with:

```bash
lsblk -f /dev/vda1
```

3. Find encrypted FS:

```bash
cd /dev/mapper
run0 cryptsetup resize luks-...
```

4. Run resize2fs:

```bash
run0 resize2fs /dev/mapper/luks-d0375771-f96a-4ef6-87d8-cf02eb807ec7
```

5. Check:

```bash
df -h /
lsblk
```

```bash
nix config show | grep download-buffer-size
```

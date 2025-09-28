### Automating Encrypted Swap Unlock

To avoid entering a separate password for the swap on every boot, you can create
a keyfile that will automatically unlock the swap once the root partition is
decrypted. This is a recommended practice.

1. Create a keyfile: Generate a random keyfile and save it to a secure location,
   like /etc/.

```bash
dd if=/dev/urandom of=/etc/swap.key bs=4096 count=1
chmod 600 /etc/swap.key
```

Add the keyfile to the LUKS header: Add the keyfile as a new key to the
encrypted swap partition.

```bash
cryptsetup luksAddKey /dev/mmcblk0p3 /etc/swap.key
```

Update `crypttab`: Modify the `/etc/crypttab` file to use the keyfile for
unlocking the swap partition instead of a password.

```bash
# Open /etc/crypttab in a text editor like nano or vim
# and change the line to:
cryptswap UUID=<your_swap_UUID> /etc/swap.key luks
```

Update `mkinitcpio`:

```bash
sudo mkinitcpio -P
```

Update `grub`:

```bash
grub-mkconfig -o /boot/grub/grub.cfg
```

Now, when you boot and enter the password for your root partition, the system
will gain access to the keyfile, which will automatically unlock the encrypted
swap without requiring a second password.

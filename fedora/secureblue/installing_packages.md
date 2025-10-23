## Installing Software

- Flatpak apps: This is the primary way that (GUI) apps get installed on Fedora
  Silverblue.

- Toolbox: Used primarily for CLI apps; development, debugging tools, etc., but
  also has support for graphical apps.

- Package layering: Most Fedora packages can be installed on the system with the
  help of package layering. By default the system operates in pure image mode,
  but package layering is useful for things like libvirt, drivers, etc.

You typically use Flatpak for GUI apps.

```bash
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
```

Check remotes to verify:

```bash
flatpak remotes
```

Install Yazi:

Search for yazi, this doesn't work or takes forever

```bash
flatpak update --appstream
flatpak search yazi
```

Had to ask an LLM for this as it doesn't show up on Flathub, I'm really hating
flatpak all over again.

```bash
flatpak install flathub io.github.sxyazi.yazi
```

## Installing packages not in Flathub

```bash
rpm-ostree install zsh
```

- This pulls zsh from Fedora's repos and stages it for the next boot. It might
  take a minute to download and verify.

- If you want utils too (e.g., for syntax highlighting):
  `rpm-ostree install zsh zsh-syntax-highlighting`.

Reboot to Apply:

```bash
systemctl reboot
```

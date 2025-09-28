## Installing Software

1. Clone the AUR Repository:

```bash
git clone https://aur.archlinux.org/librewolf-bin.git
cd librewolf-bin
```

2. Inspect the `PKGBUILD` file:

The `PKGBUILD` file is the recipe for building the package. Review it carefully:

- Check the source URL(s) to verify they are official and trustworthy.

- Look at any patches applied during the build.

- Review build and install instructions to understand what commands run.

- Confirm no malicious scripts or suspicious commands are enabled.

```bash
less PKGBUILD
```

3. Verify Integrity of Sources:

- Ensure the `sha256sums` or other checksums in `PKGBUILD` match the source
  files.

- If the source uses Git or other VCS, you can cross-check commit hashes or
  tags.

4. Check Additional Build Files:

- Sometimes other files accompany the `PKGBUILD` that affect building or
  installation like `.install` scripts, patch files, or `.conf` files. Review
  them all.

5. Build the Package Locally:

- If everything looks clean, use `makepkg` to build and install:

```bash
makepkg -si
```

- This compiles the software and prompts you to install it via `pacman`.

6. Once Installed, decide whether to keep the local directory for updates or
   delete it to save space.

7. Repeat for Updates:

- For updates, repeat the process: pull the latest changes, review, and rebuild.

**Important Notes**

- Trust only official or well-reviewed AUR packages.

- Avoid blindly using prebuild binaries from unknown sources. For example,
  `librewolf-bin` is the prebuilt binary version of the package.

- Review comments and votes on the AUR web page for community feedback.

- Consider using `makepkg` even if using AUR helpers, by manually downloading
  first for inspection.

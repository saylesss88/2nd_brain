---
id: archinstall
aliases:
  - Arch Install
tags:
  - arch
---

# Arch Install

**Steps:**

- Increase font size:

```bash
setfont -d
```

- Set Parallel Downloads to 10 and ILoveCandy in `/etc/pacman.conf`

- Update your mirrors with reflector:

```bash
reflector -c US -p https --age-6 --fastest 5 --sort rate --save /etc/pacman.d/mirrorlist
```

- Update `archinstall`:

First sync the database with:

```bash
pacman -Sy
```

Then run:

```bash
pacman -S archinstall
```

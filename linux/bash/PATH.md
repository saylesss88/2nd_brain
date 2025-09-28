---
id: PATH
aliases:
  - Add directories to PATH
tags:
  - bash
  - PATH
---

# Add directories to PATH

```bash
echo $PATH
```

Output:

```bash
.:/home/jr/Scripts:/home/jr/go/bin:/home/jr/.local/bin:/home/jr/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/home/jr/.local/share/flatpak/exports/bin:/var/lib/flatpak/exports/bin:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/jr/.local/share/bin
```

- Individual directories listed in the `PATH` are separated by `:`

To add a directory to the `PATH` you just need to reference the original `PATH`
value and add any new directories to the string.

```bash
PATH=$PATH:/home/jr/Scripts
export PATH
echo $PATH
```

- Now you can run any script in the `Scripts` directory from any directory by
  just typing its name.

> [!WARNING] Make sure to export the `PATH` variable for any changes to be
> available to sub-shells.

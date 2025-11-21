# Filesystem Layout

On Fedora Silverblue, the root filesystem (`/`) is mounted read-only. The `/usr`
directory and everything below is read only.

The `/etc` and `/var` directories are respectively used to store configuration
files and runtime state and are thus writable. Symlinks are used to make
traditional state-carrying directories available in their expected locations.
This includes:

- `/home` -> `/var/home`

- `/opt` -> `/var/opt`

- `/srv` -> `/var/srv`

- `/root` -> `/var/roothome`

- `/local` -> `/var/usrlocal`

- `/mnt` -> `/var/mnt`

- `/tmp` -> `/sysroot/tmp`

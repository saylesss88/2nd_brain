---
id: movingAroundTheFilesystem
aliases:
  - Moving around the filesystem
tags:
  - files
---

# Moving around the filesystem

One of the defining properties of the UNIX systems is that nearly everything you
need to identify on your system (data, commands, symbolic links, devices, and
directories) is represented by items in the filesystem.

- `/bin` : Contains common Linux user commands, such as ls, sort, date, and chmod.

- `/boot` : Has the bootable Linux kernel, initial RAM disk, and boot loader configuration
  files (GRUB).

- `/dev` : Contains files representing access points to devices on your systems. These
  include terminal devices (tty*), hard disks (hd* or sd*), RAM (ram*), and CD-ROM
  (cd\*). Users can access these devices directly through these device files; however,
  applications often hide the actual device names to end users.

- `/etc` : Contains administrative configuration files. Most of these files are plain-text files
  that, given the user has proper permission, can be edited with any text editor.

- `/home` : Contains directories assigned to each regular user with a login account. (The root
  user is an exception, using /root as his or her home directory.)

- `/media` : Provides a standard location for automounting devices (removable media in par-
  ticular). If the medium has a volume name, that name is typically used as the mount
  point. For example, a USB drive with a volume name of myusb would be mounted
  on /media/myusb.

- `/lib`Contains shared libraries needed by applications in /bin and /sbin to boot
  the system.

- `/mnt` : A common mount point for many devices before it was supplanted by the stan-
  dard /media directory. Some bootable Linux systems still use this directory to
  mount hard disk partitions and remote filesystems. Many people still use this
  directory to temporarily mount local or remote filesystems, which are not mounted
  permanently.

- `/misc`A directory sometimes used to automount filesystems upon request.

- `/opt` : A Directory structure available to store add-on application software.

- `/proc` : Contains info about system resources.

- `/root` : Represents the user's home directory. The home directory for root
  does not reside beneath `/home` for security reasons.

- `/sbin` : Contains administrative commands and daemon processes.

- `/sys` : Contains parameters for such things as tuning block storage and
  managing cgroups.

- `/tmp` : Contains temporary files and directories.

- `/usr` : User documentation, games, graphical files (X11), libraries (lib),
  and a variety of other files and directories that are not needed during the
  boot process.

- `/var` : Contains directories of data used by various applications. In particular, this is
  where you would place files that you share as an FTP server (/var/ftp) or a web
  server (/var/www). It also contains all system log files (/var/log) and spool files in
  /var/spool (such as mail, cups, and news). The /var directory contains directories
  and files that are meant to change often. On server computers, it is common to create
  the /var directory as a separate filesystem, using a filesystem type that can be
  easily expanded.

[[filePermissions.md]]
[[usingFileMatchingMetacharacters.md]]

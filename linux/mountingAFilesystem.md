---
id: Mounting a Filesystem
aliases:
  - Mounting a Filesystem
tags: []
---

# Mounting a Filesystem

On Linux, the process of attaching a filesystem to a running system is called
*mounting*.

  - When the system boots, it mounts the root filesystem.

In order to mount a filesystem, you must know the following:

  - The filesystem's device, location, or identifier.

  - The filesystem's type.

  - The *mount point* the place the current system's directory hierarchy where
    the filesystem is attached.
      
      - The mount point is always a normal directory and can be anywhere.

      - Commonly referred to as "mounting a device on a mount point".

      - To learn the current filesystem status of your system, you can run
        `mount`. 

```bash
$ mount
/dev/sda1 on / type ext4 (rw,errors=remount-ro)
proc on /proc type proc (rw,noexec,nosuid,nodev)
sysfs on /sys type sysfs (rw,noexec,nosuid,nodev)
fusectl on /sys/fs/fuse/connections type fusectl (rw)
debugfs on /sys/kernel/debug type debugfs (rw)
securityfs on /sys/kernel/security type securityfs (rw)
udev on /dev type devtmpfs (rw,mode=0755)
devpts on /dev/pts type devpts (rw,noexec,nosuid,gid=5,mode=0620)
tmpfs on /run type tmpfs (rw,noexec,nosuid,size=10%,mode=0755)
--snip--
```
Each line corresponds to one currently mounted filesystem, with items
in this order:
1. The device, such as /dev/sda3. Notice that some of these aren’t real
devices (proc, for example) but are stand-ins for real device names
because these special-purpose filesystems do not need devices.
2. The word on.
3. The mount point.
4. The word type.
5. The filesystem type, usually in the form of a short identifier.
6. Mount options (in parentheses). See Section 4.2.6 for more details.

To mount a filesystem manually, use the `mount` command like so with the
filesystem type, device, and desired mount point:

```bash
# mount -t type device mountpoint
```
For example, to mount the Fourth Extended filesystem found on the
device /dev/sdf2 on /home/extra, use this command:

```bash
# mount -t ext4 /dev/sdf2 /home/extra
```

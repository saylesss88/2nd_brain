---
id: arch_install
aliases:
  - Arch Install
tags: []
---

# Arch Install

1. Run `archinstall`

2. Mirrors `/` to search United States

3. Disc Configuration:
   Choose: `Use a best-effort default partition layout`
   /dev/sda is used for VirtualBox Usually choose the one with the most free space
   nvme is used for SSD

4. Select which filesystem your main partition should use:
    - btrfs - Newer offers snapshots (copy on write) easier to rollback
      Default -Use compression
      Bootloader - GRUB
      HostName - jr-xps
      Root Password - 123456
      Add a user - This is the user you'll log in as
      jr - User Password - 123456
      Should "jr" be a superuser? - Yes

4. Audio Server - PipeWire (preferred)

5. Kernel [x] linux
          [] linux-hardened
          [] linux-lts
          [] linux-zen

6. Additional packages:
  
7. Network Configuration:

- Use NetworkManager (GUI)

8. Install

9. Would you like to chroot into the newly created installation and perform
  post-installation configuration?
  Yes, This logs you in as the root user

10. Install Desktop Environment (GDM)
   `pacman -S gnome`
   Enter

- Emoji

11. `exit`
   `shutdown -h now`

12. Reboot and `Boot existing OS`

13. Bootloader Arch Linux
  enter password
  Enable the system service for GDM  `sudo systemctl enable gdm.service`
  Enter sudo password

14. `sudo systemctl start gdm.service`

15. Boom, you're done.

[[linux_boot_process.md]]

[[arch_install_i3.md]]

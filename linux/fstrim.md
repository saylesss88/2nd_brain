---
title: fstrim
date: 2024-10-29
tags: [tag1, tag2]
---

# fstrim

Trim Support: Ensure that your SSD supports TRIM and it's enabled. This helps
the drive manage and optimize unused space, which prolongs its lifespan.

```bash
sudo hdparm -l /dev/nvme0n1p1 | grep "Trim supported"
```

2. Manually Run TRIM (one time check)

```bash
sudo fstrim -v /
```

- This command will free up unused blocks on the root filesystem. If your system
  supports TRIM, this will execute without errors and show the amount of space
  trimmed.

•  sudo systemctl status fstrim.timer
[sudo] password for jr:
Sorry, try again.
[sudo] password for jr:

```bash
•   sudo fstrim -v /
/: 451.6 GiB (484881412096 bytes) trimmed
```

3. Enable TRIM on Linux automatically

```bash
sudo systemctl enable fstrim.timer
```

Start the timer to run weekly:

```bash
sudo systemctl start fstrim.timer
```

Check the Timer Status:

```bash
sudo systemctl status fstrim.timer
```

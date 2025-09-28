---
id: knowYourSystem
aliases:
  - Know Your System
tags:
  - arch
  - performance
---

# Know Your System

The best way to tune a system is to target bottlenecks, or subsystems which
limit overall speed. The system specifications can help you identify them.

- If the computer becomes slow when large applications (LibreOffice, Firefox)
  run at the same time, check if the amount of RAM is sufficient.
  Use the following command, and check the "available" column:

```bash
free -h
```

- If boot time is slow and apps take a long time to load on first launch (only),
  then the hard drive is likely to blame. The speed of the hard drive can be
  measured using the `hdparm` command:

```bash
# hdparm -t /dev/nvme0n1
```

```bash
•   sudo hdparm -t /dev/nvme0n1

/dev/nvme0n1:
 Timing buffered disk reads: 2564 MB in  3.00 seconds = 854.34 MB/sec
```

> [!NOTE] `hdparm` indicates only the pure read speed of a hard drive, and is
> not a valid benchmark. A value higher than 40MB/s (while idle) is however
> acceptable on an average system.

- If CPU load is consistently high even with enough RAM, then try to lower CPU
  usage by disabling running daemons and/or processes. This can be done with `htop`, `pstree` or any other system monitoring tool.

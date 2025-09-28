---
title: General Recommendations
date: 2024-10-11
tags: [tag1, tag2]
---

# General Recommendations

## System Maintenance

1. Failed systemd services

```bash
systemctl --failed
```

2. Log Files

Look for errors in the log files located in `/var/log`, as well as messages
logged in the systemd journal:

```bash
journalctl -b
```

- This checks for messages from this boot

```bash
journalctl -b -1  # last boot
```

[[systemdJournal.md]]

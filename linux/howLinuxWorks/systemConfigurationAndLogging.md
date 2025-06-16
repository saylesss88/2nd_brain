---
title: System Configuration and Logging
date: 2024-10-09
tags: [tag1, tag2]
---

# System Configuration and Logging

Check your Log setup

1. Check for `journald` by running `journalctl`.

2. Look in the `/var/log` for logfiles. There are a few files maintained by
   other services, such as `wtmp` and `lastlog`, the logfiles that utilities
   such as last and lastlog access in order to get login records.

## Searching and Monitoring Logs

Reverse the order of logs to see the most recent logs first with `journalctl -r`
but there are much better ways of finding logs.

**Filtering by Time**

`journalctl -S -4h`

---
id: journalctl
aliases:
  - Intro to the journalctl Command
tags: []
---

# Intro to the journalctl Command

The `journalctl` command is a utility that allows users to view and interact
with systemd journal logs.

  These logs include info from the kernel, initrd, system services, applications
  and systemd itself. 

  The logs are stored in binary format and can be queried efficiently using
  `journalctl`.

The basic syntax of the `journalctl` command is as follows:

```bash
journalctl [options] [matches...]
```

## Viewing and Filtering Logs with `journalctl`

Output from the `journalctl` command without any options can be a little
overwhelming.

  To make navigation easier, the output is automatically paginated using the
  less command.

  Use the Enter key to scroll line by line, or the Space key to scroll page by
  page. To exit press q.

To view logs in reverse chronological order (ie.newest entries first) use the `-r`
option:

```bash
journalctl -r
```

You can limit the number of displayed log entries by using the `-n` option
followed by the number of entries you want to display:

```bash
journalctl -n 10
```

Logs can also be filtered by priority. Log levels range from 0 (emergency) to 7
(debug), with lower numbers indicating higher priority. Use the `-p` option to
view logs of a specific level and above. For example, to view error, critical,
alert, and emergency logs use:

```bash
journalctl -p 3
```

To view logs from a specific system service, use the `-u` flag followed by the
service name:

```bash
journalctl -u apache2.service
```

### Working with Boot Logs

The journalctl command allows users to view logs from specific boot sessions.
Each boot is logged separately, making it easy to isolate logs related to a
particular boot. To see a list of all the logs from previous boots, use the
--list-boots option:
```bash
journalctl --list-boots
```

To view logs from the previous system boot, use the -b option with the number
-1. Specify a different number to view logs from older boots:

```bash
journalctl -b -1
```

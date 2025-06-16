---
title: "Admin Log Files and systemd Journal"
alias:
"/linux/theLinuxBible/learningSystemAdministration/adminLogFiles+systemdJournal"
---

# Admin Log Files and systemd Journal

One of the things that Linux does well is keep track of itself. This is a good
thing when you consider how much is going on in a complex operating system.

> [!NOTE] For Linux systems that don’t use the systemd facility, the main utility for logging error
> and debugging messages is the rsyslogd daemon. (Some older Linux systems use sys-
> logd and syslogd daemons.) Although you can still use rsyslogd with systemd
> systems, systemd has its own method of gathering and displaying messages called the
> systemd journal (journalctl command).

## Using journalctl to view the systemd journal

---
id: anacron
aliases:
  - anacron
tags: []
---

# anacron

If the Linux system is turned off at the time a job is scheduled to run, the job
won't run.

- The `cron` program doesn't retroactively run missed jobs when the system is
  turned back on, this is where `anacron` comes in.

Anacron is a daemon that runs scheduled jobs when the system is turned back on.

- This feature is often ussed for scripts that perform routine log maintenance.

The `anacron` program deals only with programs located in the `cron`
directories, such as `/etc/cron.monthly`.

A timestamp file exists for each `cron` directory and is located in
`/var/spool/anacron/`:

```bash
sudo cat /var/spool/anacron/cron.monthly
20150626
```

The basic format for an `anacron` entry is:

```bash
period delay identifier command
```

---
id: cronTable
aliases:
  - cron table
tags: []
---

# cron table

The format for the `cron` table is:

```bash
min hour dayofmonth month dayofweek command
```

- The `cron` table allows you to specify entries as specific values, ranges of
  values (1-5), or as a wildcard character (`*`).

If you want to run a command at 10:15 every day:

```bash
15 10 * * * command
```

- The wildcard character used in the dayofmonth, month, and dayofweek fields
  indicates that `cron` will execute the command every day of every month at
  10:15.

To specify a command run at 4:15PM every Monday:

```bash
15 16 * * 1 command
```

- You can specify the dayofweek entry as either a three-character text value (mon, tue,
  wed, thu, fri, sat, sun) or as a numeric value, with 0 being Sunday and 6 being Saturday.

[[anacron.md]]

---
id: aconfmgr
aliases:
  - aconfmgr
tags:
  - arch
  - config manager
---

# aconfmgr

First run:

Run `aconfmgr save` to transcribe the system's configuration to the
configuration directory.

This creates the file `99-unsorted.sh` in the configuration directory, usually
`~/.config/aconfmgr/99-unsorted.sh`.

## Ignore files

```bash
IgnorePath '/path/to/dir/*'
```

## Maintenance

The configuration directory should be version controlled. Ideally, the file
`99-unsorted.sh` should not be versioned, it will only be created when the
current configuration does not reflect the current system state.

- Periodic maintenance consists of running `aconfmgr save`; if this results in
  uncommitted changes to the configuration directory, then there are unaccounted
  system changes. The changes should be reviewed, sorted, documented, committed
  and pushed.

### Restoring

To restore a system to its earlier state, or to set up a new system, simply make
sure the correct configuration is in the configuration directory and run
`aconfmgr apply`.

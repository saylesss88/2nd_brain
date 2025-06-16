---
id: settingProcessorPriorityWithNice
aliases:
  - settingProcessorPriorityWithNice
tags: []
---

# settingProcessorPriorityWithNice

When the kernel tries to decide which running processes get access to the CPUs
on your system, one of the things it takes into account is the nice value set on
the process.

- By default the nice value is 0.

- The lower the nice value, the more access to the CPUs the process has.

- Regular users can only set positive nice values (0 to 19) and can only raise
  the nice value they cannot lower it.

- The root user can set the nice value on any process to any valid value, up or
  down.

You can use the `nice` command to run a command with a different nice value.
When a process is running, you can change the nice value using the `renice`
command, along with the process ID of the process.

```bash
# nice -n +5 updatedb &
```

- Then you can check that the nice value was changed with the `top` command.

Here is how you could change the nice value to `-5`:

```bash
# renice -n -5 20284
```

- The nice value for the process ID 20284 is now -5.

- If you ran the `top` command again, you would probably see that the updatedb
  command is near the top of the list.

[[limitingProcessesWithCgroups.md]]

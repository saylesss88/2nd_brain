---
id: managingRunningProcesses
aliases:
  - managingRunningProcesses
tags: []
---

# managingRunningProcesses

Multitasking means that many programs can be running at the same time. An
instance of a running program is referred to as a _process_.

- From a shell, you can launch processes and then pause, stop, or kill them. You
  can also put them in the background or foreground.

  - A process is identified on the system by what is referred to as a process ID
    (PID). That PID is unique for the current system

> [!NOTE] Commands that display information about running processes get most of that information from raw data stored in
> the /proc file system. Each process stores its information in a subdirectory of /proc, named after the process
> ID of that process. You can view some of that raw data by displaying the contents of files in one of those directories
> (using cat or less commands).

## Listing Processes

- The `ps` command is the oldest and most common command for listing processes
  currently running on your system.

- The `top` command provides a more screen-oriented approach to listing
  processes, and it can also be used to change the status of processes.

  - If you're using Gnome, you can use the System Monitor tool to provide a
    graphical means of working with processes.

### Listing processes with ps

```bash
ps u
```

- The `u` option asks that usernames be shown as well as other information such
  as the time the process started and memory and CPU usage for processes
  associated with the current user.

- The processes shown are associated with the current terminal (tty1).

- The concept of a terminal comes from the old days when people
  worked exclusively from character terminals, so a terminal typically represented a single
  person at a single screen. Nowadays, you can have many “terminals” on one screen by
  opening multiple virtual terminals or Terminal windows on the desktop.

- [~] `procs` is a more modern alternative to `ps` that has syntax highlighting
  and automatically pages the output with less.

[[listingAndChangingProcessesWithTop.md]]

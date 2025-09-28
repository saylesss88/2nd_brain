---
id: usingForeground&BackgroundCommands
aliases:
  - usingForeground&BackgroundCommands
tags: []
---

# usingForeground&BackgroundCommands

```bash
$ jobs

[1] Stopped (tty output) vi /tmp/myfile
[2] Running
find /usr -print > /tmp/allusrfiles &
[3] Running
nroff -man /usr/man2/* >/tmp/man2 &
[4]- Running
nroff -man /usr/man3/* >/tmp/man3 &
[5]+ Stopped
nroff -man /usr/man4/* >/tmp/man4
```

You can bring any of the commands on the output of the `jobs` list to the
foreground. for example to edit `myfile` again enter:

```bash
fg %1
```

- As a result, the `vi` command opens again. All text is as it was when you
  stopped the `vi` job.

- [!] Before you put a text processor, word processor, or similar program in the
  background, make sure that you save your file. It’s easy to forget that you have
  a program in the background, and you will lose your data if you log out or the
  computer reboots.

To refer to a background job (to cancel or bring it to the foreground), use a
percent sign (`%`) in front of the job number. You can also:

`%` : Refers to the most recent command put into the background (indicated by the
plus sign when you type the jobs command). This action brings the command to
the foreground.
`%string` : Refers to a job where the command begins with a particular string of characters.
The string must be unambiguous. (In other words, typing %vi when there are two
vi commands in the background results in an error message.)

`%?string` : Refers to a job where the command line contains a string at any point. The string
must be unambiguous or the match fails.
`%--` : Refers to the job stopped before the one most recently stopped.

If a command is stopped, you can start it running again in the background using the bg
command. For example, refer back to job 5 from the jobs list in a previous example:
`[5]+ Stopped nroff -man /usr/man4/* >/tmp/man4`.

Enter the following:
`$ bg %5`.

After that, the job runs in the background. Its jobs entry appears as follows:
`[5] Running nroff -man /usr/man4/* >/tmp/man4 &`.

The `kill` and `killall` commands can actually be used to send any valid signal
to a running process. Besides telling a process to end, a signal might tell a
process to reread configuration files, pause (stop), or continue after being
paused, just to name a few possibilities. Signals are represented by both numbers
and names. Signals that you might send most commonly from a command include
SIGKILL (9), SIGTERM (15), and SIGHUP (1). The default signal is SIGTERM, which
tries to terminate a process cleanly. To kill a process immediately, you can use
SIGKILL. The SIGHUP signal can, depending on the program, tell a process to
reread its configuration files. SIGSTOP pauses a process, while SIGCONT continue
s a stopped process.

For example, you run the `top` command to see that the `bigcommand` process is
consuming most of your processing power:

```bash
PID    USER  PR   NI  VIRT  RES   SHR S   %CPU %MEM    TIME+  COMMAND
10432  chris 20   0  1104  1104  1056 R   99.9  0.0   0:00.00 bigcommand
```

- Here `bigcommand` is consuming 99.9% of the CPU. You can use the `kill`
  command so other processes have a shot at the CPU.

```bash
kill 10432
kill -15 10432
kill -SIGKILL 10432
```

- The default signal sent by kill is `15 (SIGTERM)`, so the first two have
  exactly the same results.

- Sometimes `SIGTERM` doesn't kill a process, so you may need a `SIGKILL` to
  kill it. Instead of `SIGKILL`, you can use `-9` to get the same result.

If, for example, something on your GNOME desktop were
corrupted, you could send the gnome-shell a SIGHUP signal to reread its configuration
files and restart the desktop. If the process ID for gnome-shell were 1833, here are two
ways you could send it a SIGHUP signal:

```bash
# kill -l 1833
# killall -HUP gnome-shell
```

## Using killall to signal processes by name

With the `killall` command, you can signal processes by name instead of process
ID.

- The advantage is that you don’t have to look up the process ID of the process
  that you want to kill. The potential downside is that you can kill more processes
  than you mean to if you are not careful. (For example, typing killall bash may
  kill a bunch of shells that you don’t mean to kill.)

- Like the kill command, killall uses SIGTERM (signal 15) if you don’t explicitly enter
  a signal number.

[[settingProcessorPriorityWithNice.md]]

---
id: managingBackground&ForegroundProcesses
aliases:
  - managingBackground&ForegroundProcesses
tags: []
---

# managingBackground&ForegroundProcesses

If you are using Linux over a network or from a _dumb terminal_ (a monitor that
allows only text input with no GUI support), your shell may be all that you
have.

- Although the bash shell doesn’t include a GUI for running many programs at once, it does
  let you move active programs between the background and foreground. In this way, you
  can have lots of stuff running and selectively choose the one you want to deal with at
  the moment.

You can place an active program in the background in several ways:

- You can add an ampersand (`&`) at the end of the program name to make it run
  in the background.

- You can also use the `at` command to run commands in such a way that they are
  not connected to the shell.

- To stop a running command and put it in the background, press `CTRL+Z`. After
  the command is stopped, you can either bring it back with `fg` or start it
  running in the background (the `bg` command).

- Keep in mind that any command running in the background might spew output
  during commands that you run subsequently from that shell. Just press `CTRL+L`
  to clear the output.

> [!TIP] To avoid having the output appear, you should have any process running
> in the background send its output to `/dev/null` (add `2>/dev/null` to the end
> of the command line).

```bash
find /usr > /tmp/allusrfiles &
[3] 15971
```

This command finds all files starting with `/usr`, prints those filenames, and
puts those names in the file `/tmp/allusrfiles`. The (`&`) runs that command
line in the background.

- Notice the job number `[3]`, and the process ID `15971` are displayed when the
  command is launched.

- To check which commands you have running in the background, use the `jobs`
  command:

```bash
jobs

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

- The first job shows a text-editing command (vi) that I placed in the background and
  stopped by pressing Ctrl+Z while I was editing. Job 2 shows the find command I just ran.
  Jobs 3 and 4 show nroff commands currently running in the background. Job 5 had been
  running in the shell (foreground) until I decided too many processes were running and
  pressed Ctrl+Z to stop job 5 until a few processes had completed.

- The plus sign (+) next to number 5 shows that it was most recently placed in the
  background. The minus sign (-) next to number 4 shows that it was placed in the
  background just before the most recent background job. Because job 1 requires terminal
  input, it cannot run in the background. As a result, it is Stopped until it is brought to the
  foreground again.

> [!TIP] You can also use `jobs -l` to list all of the jobs in the background.

[[usingForeground&BackgroundCommands.md]]

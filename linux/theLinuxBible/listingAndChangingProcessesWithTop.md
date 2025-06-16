---
id: listingAndChangingProcessesWithTop
aliases:
  - listingAndChangingProcessesWithTop
tags: []
---

# listingAndChangingProcessesWithTop

The `top` command provides a screen-oriented means of displaying processes
running on your system. The default is to display processes based on how much
CPU time they are consuming.

- After you identify a misbehaving process, you can use `top` to kill
  (completely end) or renice (change the priority of) the process.

  - If you want to be able to kill or renice the process, you need to run `top
` as root.

  - General info about your system appears at the top of the `top` output,
    followed by info about each running process.

  - At the top, you can see how long the system has been up, how many users are
    currently logged in, and how much demand there has been on the system for
    the past 1, 5, and 10 minutes.

  - Other general information includes how many processes (tasks) are currently
    running, how much CPU is being used, and how much RAM and swap are available
    and being used.

  - Following the general information are listings of each process, sorted by what percent of the
    CPU is being used by each process. All of this information is redisplayed every 5 seconds,
    by default.

  - The following list includes actions that you can do with `top`:

    - Press `h` to see help options
    - `M`: sort by memory usage instead of CPU usage, press `P` to reverse.
    - `1` : toggle showing CPU usage of all your CPUs
    - `R` : reverse sort your output.
    - `u` : and enter username to display processes only for that user.

## Renicing and Killing Processes

**Renicing a process**: Note the process ID of the process you want to renice
and press `r`. When the `PID to renice` message appears, type the process ID.
When prompted to `Renice PID to value`, type in a number from `-20` to `19`.

**Killing a process**: Note the process ID of the process you want to kill and
press `k`. Type `15` to terminate cleanly or `9` to terminate outright.

### Listing processes with System Monitor

You sort processes by clicking columns.

System Monitor displays processes for the current user in order by memory use.

- By default, only running processes associated with your user account are
  displayed.

  - Those are listed alphabetically first.

  - You can resort the processes by clicking any of the field headings (forward
    and reverse).

  - Click `%CPU` to see which processes are using the most CPU.

  - Click `%MEM` to see which processes are using the most memory.

  - You can change your processes in various ways by right-clicking a process name and select-
    ing from the menu that appears.

Here are some of the things you can do to a process from the menu you clicked:

**Stop**: Pauses the process so that no processing occurs until you select Continue Process.
(This is the same as pressing Ctrl+Z on a process from the shell.)
Continue: Continues running a paused process.

**End**: Sends a Terminate signal (15) to a process. In most cases, this terminates the pro-
cess cleanly.

**Kill**: Sends a Kill signal (9) to a process. This should kill a process immediately, regard-
less of whether it can be done cleanly.

**Change Priority**: Presents a list of priorities from Very Low to Very High. Select Custom
to see a slider bar from which you can renice a process. Normal priority is 0. To get
better processor priority, use a negative number from –1 to –20. To have a lower pro-
cessor priority, use a positive number (0 to 19). Only the root user can assign neg-
ative priorities, so when prompted you need to provide the root password to set a
negative nice value.

**Memory Maps**: Lets you view the system memory map to see which libraries and other
components are being held in memory for the process.
Open Files: Lets you view which files are currently being held open by the process.

**Properties**: Lets you see other settings associated with the process (such as security
context, memory usage, and CPU use percentages).

[[managingBackground&ForegroundProcesses.md]]

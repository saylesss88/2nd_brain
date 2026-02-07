# Async optimizations

Find the pid of the running process:

1. Find the processes whos command line contains `persway`

```bash
ps aux | grep persway
# Output:
jr          2348  0.0  0.0 682400  5204 ?        Ssl  18:08   0:00 persway daemon -w -e [tiling] opacity 1 -f [tiling] opacity 0.9; opacity 1 -l mark --add _prev --default-layout spiral
jr          5771  0.0  0.0 13416146708 5988 pts/0 S+  18:14   0:00 grep --color=auto persway
```

2.

```bash
ps -eo pid,pcpu,pmem,comm | grep persway
2348 0.0 0.0 persway
```

- `pid`(process ID),
- `pcpu`(CPU usage),
- `pmem`(RAM usage),
- `comm`(the command name)

3.  `ls -l /proc/2348/fd | grep socket`

This is the key part of your investigation.

- `/proc/2348/fd` is the directory of open file descriptors for process
  PID 2348.

- `ls -l` lists them with symbolic links.

- `grep socket` filters only the entries that are sockets.

Output:

```bash
ls -l /proc/2348/fd | grep socket
lrwx------@ - jr  4 Feb 18:14 1 -> socket:[27827]
lrwx------@ - jr  4 Feb 18:14 2 -> socket:[27827]
lrwx------@ - jr  4 Feb 18:14 6 -> socket:[27233]
lrwx------@ - jr  4 Feb 18:14 7 -> socket:[27234]
lrwx------@ - jr  4 Feb 18:14 8 -> socket:[27233]
lrwx------@ - jr  4 Feb 18:14 9 -> socket:[27235]
lrwx------@ - jr  4 Feb 18:14 13 -> socket:[27236]
lrwx------@ - jr  4 Feb 18:14 14 -> socket:[27237]
lrwx------@ - jr  4 Feb 18:14 15 -> socket:[27238]
lrwx------@ - jr  4 Feb 18:14 16 -> socket:[27239]
lrwx------@ - jr  4 Feb 18:14 17 -> socket:[23305]
```

- The left side numbers (`1`, `2`, `6`, `7`, etc.) are file descriptor numbers.

- `socket:[27233]`` etc. is the internal kernel socket ID.

- Each `socket:[...]` represents one open socket.

So this line:

```bash
ls -l /proc/2348/fd | grep socket | wc -l
11
```

is counting how many open sockets that `persway` instance has. `11` open sockets
right now.

4. What 11 sockets means for your async tokio / `WindowFocus` changes For
   `persway` talking to sway via Unix domain sockets (like `swayipc`):

- A well‑behaved long‑running IPC client usually has:
  - 1 socket to `sway` (persistent connection).

  - Maybe 1–2 extra for logging, tokio internals, or other clients.

What you’re seeing (`11` sockets) suggests:

- Before your `WindowFocus` refactoring, the app might have been:
  - Opening a new socket for every window‑focus event (or for every
    `WindowFocus::run` task).

  - Those sockets weren’t closing immediately, or were being GCed slowly, so you
    ended up with many open sockets over time.

- After your refactoring (single WindowFocus with a persistent connection),
  you’d expect:
  - That number to be much lower (ideally 1–3).

  - If you re‑run that same check after the changes, you’re essentially
    validating that you’re no longer creating a new socket for every event.

---

**How to interpret this as a “check”**

You’re doing this:

1. Before change:

- Measure how many sockets a running persway has during normal usage.

2. After change:

- Restart persway, exercise the same focus‑switching, and then:

```bash
ls -l /proc/$(pgrep persway)/fd | grep socket | wc -l
```

Compare the number.

If it goes from ~10–100+ back down to ~1–3, you’ll have concrete evidence that:

- You’re no longer creating a new socket per event,

- The CPU spike and flashing were likely caused by that socket‑spam‑induced IPC
  feedback loop.

---

**TL;DR**

- `ps aux | grep persway` → find PID.

- `ps -eo ...` → check CPU/memory.

- `ls -l /proc/2348/fd | grep socket` → see all open sockets for that PID.

- `... | wc -l` → count open sockets (11 means 11 open sockets right now).

You’re checking: is `persway` leaking sockets because of frequent reconnects?
You’re about to use that number as a metric to see if your `WindowFocus`
refactor fixed it

---
id: searchingInFilesWithGrep
aliases:
  - searchingInFilesWithGrep
tags: []
---

# searchingInFilesWithGrep

- If you want to search for files that contain a certain search term, you can
  use `grep`. With `grep`, you can search a file or search a whole directory
  structure of files recursively.

  - When you search, you can have every line containing the term printed on your
    screen (standard output) or just list the names of the files that contain the
    search term.

  - By default `grep` searches text in a case-sensitive way, although you can do
    case-insensitive searches with the `-i` option.

  - Instead of just searching files you can also have `grep` search standard
    output. So, if a command turns out lots of text and you want to find only
    lines that contain certain text, you can use `grep` to filter just what you
    want.

Some examples of `grep` used to find text strings in one or more files:

```bash
grep desktop /etc/services
desktop-dna      2763/tcp
desktop-dna      2763/udp
grep -i desktop /etc/services # case-insensitive
desktop-dna      2763/tdp
desktop-dna      2763/udp
```

- In the first example, a `grep` for the word `desktop` in the `/etc/services`
  file turned up 2 lines.

To search for lines that don't contain a selected text string, use the `-v`
option. In the following all lines from the `/etc/services` file are displayed
except those containing the text `tcp` (case-insensitive):

```bash
grep -vi tcp /etc/services
```

- To do recursive searches, use the `-r` option and a directory as an argument.
  The following includes the `-l` option, which just lists files that include the search text, without showing the actual lines of text.

```bash
grep -rli peerdns /usr/share/doc/
/usr/share/doc/dnsmasq-2.66/setup.html
/usr/share/doc/initscripts-9.49.17/sysconfig.txt
...
```

To search the output of a command for a term, you can pipe the output to grep.
In this example, I know that IP addresses are listed on output lines from the
`ip` command that include the string `inet`, so i use `grep` to display those
lines:

```bash
ip addr show | grep inet
inet 127.0.0.1/8 scope host lo
inet6 ::1/128 scope host noprefixroute
inet 192.168.0.52/24 brd 192.168.0.255 scope global wlan0
inet6 2601:881:8100:96b0:4ae2:44ff:fef5:a3cb/64 scope global dynamic mngtmpaddr proto kernel_ra
inet6 fe80::4ae2:44ff:fef5:a3cb/64 scope link proto kernel_ll
```

[[managingRunningProcesses.md]]

---
id: head_tail
aliases: []
tags:
  - commands
---

By default, `head` and `tail` display the first and last 10 lines of a file
respectively. You can change this by using the `-n` or `--lines` option.

```bash
head -n 5 ls-output.txt
drwxr-xr-x     - root 23 Aug 12:49 core_perl
drwxr-xr-x     - root 23 Aug 12:49 db5.3
drwxr-xr-x     - root 11 Feb 15:15 site_perl
drwxr-xr-x     - root 23 Aug 12:49 vendor_perl
lrwxrwxrwx     - root  9 Aug 04:20 2to3 -> 2to3-3.12
```
These can be used in pipelines as well:

```bash
ls /usr/bin | tail -n 5
lrwxrwxrwx     - root 10 May 18:14 zstdmt -> zstd
.rwxr-xr-x  104k root 25 Aug  2023 zvbi-atsc-cc
.rwxr-xr-x   14k root 25 Aug  2023 zvbi-chains
.rwxr-xr-x  129k root 25 Aug  2023 zvbi-ntsc-cc
.rwxr-xr-x   47k root 25 Aug  2023 zvbid
```
`tail` has an option that allows you to view files in real time. This is useful
for watching the progress of log files as they're being written.

```bash
sudo tail -f /var/log/messages
```
[[tee -- Read from Stdin and Output to Stdout and Files.md]]

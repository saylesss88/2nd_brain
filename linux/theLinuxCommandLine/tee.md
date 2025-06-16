---
id: tee -- Read from Stdin and Output to Stdout and Files
aliases: []
tags:
  - commands
---

The `tee` program reads standard input and copies it to both standard output
(allowing the data to continue down the pipeline) and to more files. This is
useful for capturing a pipeline's contents at an intermediate stage of
processing.

Here we use `tee` to capture the entire directory listing to the file ls.txt
before `grep` filters the pipeline's contents:

```bash
❯ ls /usr/bin | tee ls.txt | grep zip
.rwxr-xr-x   31k root 28 Apr 15:12 bsdunzip
lrwxrwxrwx     - root 17 Mar 18:29 bunzip2 -> bzip2
lrwxrwxrwx     - root 17 Mar 18:29 bzcat -> bzip2
.rwxr-xr-x   39k root 17 Mar 18:29 bzip2
.rwxr-xr-x   14k root 17 Mar 18:29 bzip2recover
.rwxr-xr-x   27k root  8 Jun 16:12 funzip
.rwxr-xr-x  2.3k root 24 Jul 02:42 gunzip
.rwxr-xr-x   93k root 24 Jul 02:42 gzip
.rwxr-xr-x   14k root  2 Jul 15:38 hunzip
.rwxr-xr-x   19k root  2 Jul 15:38 hzip
.rwxr-xr-x   27k root  4 Aug 18:25 libdeflate-gunzip
.rwxr-xr-x   27k root  4 Aug 18:25 libdeflate-gzip
lrwxrwxrwx     - root 30 Jun 17:39 lrunzip -> lrzip
lrwxrwxrwx     - root 30 Jun 17:39 lrz -> lrzip
lrwxrwxrwx     - root 30 Jun 17:39 lrzcat -> lrzip
.rwxr-xr-x  258k root 30 Jun 17:39 lrzip
.rwxr-xr-x  110k root  1 Mar 17:19 lzip
lrwxrwxrwx     - root  2 Jun 08:50 mzip -> mtools
.rwxr-xr-x   14k root  7 Apr 11:13 orcus-zip-dump
.rwxr-xr-x  170k root  8 Jun 16:12 unzip
.rwxr-xr-x   76k root  8 Jun 16:12 unzipsfx
.rwxr-xr-x  237k root  6 Oct  2023 zip
.rwxr-xr-x  102k root  6 Oct  2023 zipcloak
.rwxr-xr-x   27k root 12 Jul 17:34 zipcmp
.rwxr-xr-x  3.0k root  8 Jun 16:12 zipgrep
.rwxr-xr-x  170k root  8 Jun 16:12 zipinfo
.rwxr-xr-x   15k root 12 Jul 17:34 zipmerge
.rwxr-xr-x   98k root  6 Oct  2023 zipnote
.rwxr-xr-x  102k root  6 Oct  2023 zipsplit
.rwxr-xr-x   35k root 12 Jul 17:34 ziptool
```
[[Expansion.md]]

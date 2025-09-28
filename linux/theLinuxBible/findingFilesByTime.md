---
id: findingFilesByTime
aliases:
  - findingFilesByTime
tags:
  - commands
---

# findingFilesByTime

- Date and time stamps are stored for each file when it is created, when it is
  accessed, when its content is modified, or when its metadata is changed.

  - Metadata includes owner, group, time stamp, file size, permissions, and
    other info stored in the file's inode.

A few examples:

- You just changed the contents of a config file, and you can't remember which
  one. So you search `/etc` to see what has changed in the past 10 minutes:

```bash
find /etc/ -mmin -10
```

- You suspect that someone hacked your system three days ago. So, you search the
  system to see if any commands have had their ownership or permissions changed
  in the past three days:

```bash
find /bin /usr/bin /sbin /usr/sbin -ctime -3
```

- You want to find files in your FTP server (/var/ftp) and web server (/var/www)
  that have not been accessed in more than 300 days so that you can see if any
  need to be deleted:

```bash
find /var/ftp /var/www -atime +300
```

- The time options (-atime, -ctime, and -mtime) enable you to search based on
  the number of days since each file was accessed, changed,or had its metadata
  changed. The min options (-amin, -cmin, and -mmin) do the same in minutes.

  - Numbers that you give as arguments to the `min` and `time` options are
    preceded by a hyphen (to indicate a time from the current time to that
    number of minutes or days ago) or a plus sign (to indicate a time from the
    number of minutes or days ago and older).

  - With no hyphen or plus, the exact number is matched.

## Use 'not' and 'or' when finding files

- With the `-not` and `-or` options, you can further refine your searches.

  - There is a shared directory called `/var/allusers`. This command line
    enables you to find files that are owned by either `joe` or `chris`:

```bash
find /var/allusers \( -user joe -o -user chris \) -ls
$ find /var/allusers \( -user joe -o -user chris \) -ls
679967
0 -rw-r--r-- 1 chris chris
0 Dec 31 12:57
/var/allusers/myjoe
679977 1812 -rw-r--r-- 1 joe
joe
4379 Dec 31 13:09
/var/allusers/dict.dat
679972
0 -rw-r--r-- 1 joe
sales
0 Dec 31 13:02
/var/allusers/one
```

- This searches for files owned by the user `joe`, but only those that are not
  assigned to the group `joe`:

```bash
find /var/allusers/ -user joe -not -group joe -ls
679972 0 -rw-r--r-- 1 joe sales 0 Dec 31 13:02 /var/allusers/one
```

- You can add multiple requirements to your searches. Here, a file must be owned
  by the user `joe` and must also be more that 1MB in size:

```bash
find /var/allusers/ -user joe -and -size +1M -ls
679977 1812 -rw-r--r-- 1 joe root 1854379 Dec 31 13:09
/var/allusers/dict.dat
```

### Finding files and Executing Commands

With the -exec option, the command you use is exe-
cuted on every file found, without stopping to ask if that’s okay. The -ok option
stops at each matched file and asks whether you want to run the command on it.

- The advantage of using -ok is that, if you are doing something destructive, you can make
  sure that you okay each file individually before the command is run on it. The syntax for
  using -exec and -ok is the same:

```bash
find [options] -exec command {} \;
find [options] -ok command {} \;
```

- With `-exec` or `-ok`, you run `find` with any options you like in order to
  find the files you are seeking. Then you enter the option followed by the
  command you want to run on each file.

- The set of curly braces indicates where on the command line to read in each
  file that is found.

- Each file can be included in the command line multiple times.

- To end the line, you need to add a backslash and semicolon (`\;`).

- This command finds any file named `passwd` under the `/etc` directory and
  includes that name in the output of an `echo` command:

```bash
find /etc -iname passwd -exec echo "I found {}" \;
I found /etc/pam.d/passwd
I found /etc/passwd
```

[[searchingInFilesWithGrep.md]]

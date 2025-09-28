---
id: understandingCommandSyntax
aliases: []
tags:
  - commands
---

You can group single-letter options together or precede each with a hyphen to
use more than one option at a time.

```bash
ls -l -a -t
ls -lat
```

- In both commands `ls` is run with the `-l`(long listing), `-a`(all), and
  `-t`(list by time).

- Many commands also accept *arguments* after certain options are entered or at
  the end of the entire command line.
 
  - An **argument** is an extra piece of info, such as a file name, directory,
      username, device, or other item, that tells the command what to act on.

  - For example, `cat /etc/passwd` will print the contents of the
      `/etc/passwd` to the screen. In this case, the argument is `/etc/passwd`.

  - Usually you can have as many arguments as you want, limited only by the
      total number of characters allowed.

  - Sometimes, an argument is associated with an option. In that case, the
    argument must immediately follow the option in the command line.

  - With single-letter options, the argument typically follows after a space.

  - For full-word options, the argument often follows an equal sign.

```bash
ls --hide=Desktop
Documents Music  Public  Videos
Downloads Pictures Templates
```

- Here, the `--hide` option is used to hide the `Desktop` folder from the list.

Here's an example of a single-letter option that is followed by an argument:

```bash
tar -cvf backup.tar /home/jr
```

- In this example, the options say to create (c) a file (f) named `backup.tar`
  that includes all of the contents of the `/home/jr` directory and its
  subdirectories and show verbose (v) messages as the backup is created.

  - Because `backup.tar` is an argument to the f option, `backup.tar` must
    immediately follow the option.

A few different date commands:

```bash
date
date +'%d/%m/%y'
28/08/24

date +'%A, %B %d, %Y'
Wendsday, August 28, 2024
```

[[LocatingCommands.md]]

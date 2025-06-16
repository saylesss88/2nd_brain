---
id: SequentialCommands
aliases: []
tags: []
---

Sommetimes you may want one command to complete before the next command starts.
You can do this by typing several commands on the command line and separating
them with semicolons (`;`):

```bash
date ; troff -me verylargedocument | 1pr ; date
```

- In this example, The author was formatting a huge document and wanted to know
  how long it would take.

   - The first command (`date`) showed the date and time before the formatting
     started.

    - The `troff` command formatted the doc and then piped the output to the
      printer.

    - When the formatting finished, the date and time were printed again (so he
      knew how long the formatting took).

Another useful command to add to the end of a long command line is `mail`. You
could add the following to the end of a command line:

```bash
; mail -s "Finished the long command" chris@example.com
```

Then, for example, a mail message is sent to the user you choose after the
command completes.

## Background Commands

Some commands take a while to complete. To avoid tying up your shell you can
have the commands run in the background by using the `&` character:

- Text formatting commands (such as `nroff` and `troff`) are examples of
  commands that can be run in the background to format a large doc.

- You may also want to create your own shell scripts that run in the background
  to check continuously for certain events to occur, such as the hard disk
  filling up or particular users logging in.

```bash
troff -me verylargedocument | 1pr &
```

- Don't close the shell until the process is completed or that kills the
  process.

[[ExpandingCommands.md]]

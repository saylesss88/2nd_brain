---
id: Command-lineRecall
aliases:
  - Command-lineRecall
tags: []
---

# Command-lineRecall

After you type a command, the entire command line is saved in your shell's
history list. The list is stored in the current shell until you exit, then it is
written to a history file, from which any command can be recalled to be run
again in your next session.

To view your history list, use the history command. Enter the command without options
or followed by a number to list that many of the most recent commands. For example:

```bash
history 8
  924  /bin/date
  925  echo $PATH
  926  which bash
  927  type -a ls
  928  bash
  929  ls /usr/bin | sort -f | less
  930  echo $OSTYPE 
  931  history 8
```

- You can recall one of the numbers to the left of the command using an
  exclamation pount (!). When using an exclamation point, the command runs blind
  without presenting an opportuninty to confirm the command you're referencing.

  - To run which bash again, you can type `!928`.

  - `!!` will run the previous command.

  - `!?string-?` Run command containing string.

```bash
!?dat?
Wed Aug 28 11:25:25 AM EDT 2024
```

- If you type `fc 928` it will open the command in your editor.
  - You can also use a range of numbers for example `fc 928-930` will list those
    commands in your editor.

- After you close your shell, the history is stored in the `.bash_history` file
  in your home directory. By default, up to 1,000 commands are stored.

> [!NOTE] Some people disable the history feature for the root user by setting
> HISTFILE shell variable to `/dev/null` or simply leaving HISTSIZE blank. This
> prevents info about the root users activities from being exploited. Also,
> because the shell history is stored permanently when the shell exits properly,
> you can prevent this by killing the shell ex. `kill -9 $SHELL_PID`

[[Connecting&ExpandingCommands.md]]

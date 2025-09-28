---
id: settingYourPrompt
aliases:
  - Setting Your Prompt
tags:
  - shell
---

# Setting Your Prompt

- Your prompt consists of a set of characters that appear each time the shell is
  ready to accept a command.

   - The `PS1` env variable sets what the prompt contains and is what you will
     interact with most of the time.

   - If your shell requires additional input, it uses the values of `PS2`,
      `PS3`, and `PS4`.

  CharactersSpecial Character
- `\!` : This shows the current command history number. This includes all previous
 
- `\#` : This shows the command number of the current command. This includes only
      the commands for the active shell.

- `\$`
This shows the user prompt ($) or root prompt (#), depending on which type
of user you are.

- `\W`
This shows only the current working directory base name. For example,
if the current working directory was /var/spool/mail, this value simply
appears as mail.

- `\[`
This precedes a sequence of nonprinting characters. This can be used to
add a Terminal control sequence into the prompt for such things as changing
colors, adding blink effects, or making characters bold. (Your Terminal deter-
mines the exact sequences available.)

- `\]`
This follows a sequence of nonprinting characters.

- `\\`
This shows a backslash.

- `\d`
This displays the day name, month, and day number of the current date, for
example, Sat Jan 23.

- `\h`
This shows the hostname of the computer running the shell.

- `\n`
This causes a newline to occur.

- `\nnn`
This shows the character that relates to the octal number replacing nnn.

- `\s`
This displays the current shell name. For the bash shell, the value
would be bash.

- `\t`
This prints the current time in hours, minutes, and seconds, for exam-
ple, 10:14:39.

- `\u`
This prints your current username.

- `\w`
This displays the full path to the current working directory.

> [!NOTE] If you are setting your prompt temporarily by typing at the shell, put
> the value of `PS1` in quotes. For example, you could type `export PS1="[\t \w]\$"`
> to see a prompt like this `[20:26:32 /var/spool]$`

- To make the change permanent, add the value of `PS1` to your `~/.bashrc` 

## Adding Environment Variables

Adding a few environment variables to your prompt will make it easier to work
with, more efficient, and effective:

- `TMOUT` : This sets how long the shell can be inactive before it bash
  automatically exits.
   - The value is the number of seconds for which the shell has not received
     input.
   - This is a nice security feature, in case you leave your desk while you're
     still logged in. `TMOUT=1800` will set the timeout to 30 minutes.

- `PATH` : If you often use directories of commands that are not in your path,
  you can permanently add them. To do this, add a `PATH` variable to your
  `~/.bashrc` file.
  - For example, to add a dir named `/getstuff/bin`, add the following:

```bash
PATH=$PATH:/getstuff/bin ; export PATH
```

- This example first reads all of the current path directories into the new
  `PATH` (`$PATH`), and then adds `/getstuff/bin` to the end of it.

- `WHATEVER` : You can create your own environment variables to provide
  shortcuts in your work. 
  - For example, if you do lots of work with files in the /work/time/files/info/
memos directory, you could set the following variable:

`M=/work/time/files/info/memos ; export M`

  - You could make that your current directory by typing `cd $M`.
  - You could run a program from that directory called `hotdog` by typing
    `$M/hotdog`. You could edit file `bun` by typing `$M/bun`

[[getInfoAboutCommands.md]]

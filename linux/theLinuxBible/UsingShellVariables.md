---
id: UsingShellVariables
aliases:
  - Using Shell Variables
tags:
  - shell
---

# Using Shell Variables

The shell itself stores info that may be useful to the user's shell session in
what are called variables.

- `$SHELL` identifies the name of the shell being used.

- `$PS1` defines your shell prompt.

- `$MAIL` identifies the location of your user's mailbox.

- You can see all variables for your current shell using the `set` command.

  - A subset of your local variables is referred to as environment variables.

  - Environment variables are variables that are exported to any new shells
    opened from the current shell. Type `env` to see all of your environment.

## Creating and using aliases

Using the `alias` command, you can effectively create a shortcut to any command
and options that you want to run later.

- You can add and list aliases using the `alias` command.

```bash
alias p='pwd ; ls -CF'
alias rm='rm -i'
```

### Common Shell Environment Variables

**Variable** | **Description**
BASH | Contains the full pathname of the bash command
BASH_VERSION | Contains the version of bash
EUID | The effective user ID of the current user
FCEDIT | If set, indicates the editor used by the fc command
HISTFILE | Contains the pathname of the user's history file
HISTFILESIZE | Contains the maximum number of lines stored in the history

- Variables can contain the output of a command or command sequence.

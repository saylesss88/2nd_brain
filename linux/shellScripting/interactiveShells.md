---
id: interactiveShells
aliases:
  - interactiveShells
tags: []
---

# interactiveShells

The interactive login shell in Bash is a special type of shell that is launched
when you log in to a system. It provides a command-line interface where you can
interact with the system and execute commands.

Here are some key characteristics of an interactive login shell:

    Environment variables: The shell sets various environment variables when it starts, such as HOME, PATH, SHELL, and USER. These variables provide information about the current user, the user's home directory, and the search path for commands.

    Command history: The shell keeps a history of the commands you've entered, allowing you to recall and edit them using the up and down arrow keys.

    Aliases: You can create aliases to shorten long or complex command sequences.

    Prompt: The shell displays a prompt, usually a dollar sign ($), to indicate that it's ready for you to enter a command.

    Job control: The shell allows you to manage background jobs and foreground processes.

In contrast to an interactive login shell, a non-interactive shell is typically
used for executing scripts or commands without direct user interaction.

Here's a simple example of an interactive login shell in action:

When you log in to a system, you'll see a prompt like this:

```bash
user@host:~$
```

You can then enter commands, such as:

```bash
ls -l
cd Documents
vim myfile.txt
```

The shell will execute these commands and display the results.

Note: The specific behavior and features of the interactive login shell may
vary depending on the operating system and shell configuration. However, the
general principles outlined above apply to most interactive login shells.

## Invoked

Interactive means you can enter commands. The shell is not running because a
script has been activated. A login shell means that you got the shell after
authenticating the system, usually by entering your username and password.

Files read:

    - /etc/profile

    - ~/.bash_profile, ~/.bash_login, ~/.profile

    ~/.bash_logout upon logout

### Non-Login Shell

A non-login shell means that you didn't have to authenticate the system. Like
when you open a terminal with an icon, or menu item, that's a non-login shell.

Files read:

    - ~/.bashrc

This file is usually referred to in `~/.bash_profile` or `~/.profile`:

```bash
if [ -f ~/.bashrc ]; then . ~/.bashrc; fi
```

All scripts use non-interactive shells. They are programmed to do certain tasks
and cannot be instructed to do other jobs than those specified in the script.

Files read:

    - defined by BASH_ENV

`PATH` is not used to search for this file, so if you want to use it, best refer
to it by giving the full path name.

#### Interactive Shells

An interactive shell generally reads from, and writes to, a user's terminal:
input and output are connected to a terminal.

- Bash interactive behavior is started when the `bash` command is called upon
  without non-optional arguments.

**Is this shell interactive?**

Test by looking at the content of the special parameter `-`, it contains an `i`
when the shell is interactive.

```bash
echo $-
himBHs
```

- In non-interactive shellls, the prompt, `PS1`, is unset.

Differences in interactive mode:
• Bash reads startup files.
• Job control enabled by default.
• Prompts are set, PS2 is enabled for multi-line commands, it is usually set to ">". This is also the
prompt you get when the shell thinks you entered an unfinished command, for instance when you
forget quotes, command structures that cannot be left out, etc.
• Commands are by default read from the command line using readline.
• Bash interprets the shell option ignoreeof instead of exiting immediately upon receiving EOF
(End Of File).
• Command history and history expansion are enabled by default. History is saved in the file pointed to
by HISTFILE when the shell exits. By default, HISTFILE points to ~/.bash_history.
• Alias expansion is enabled.
• In the absence of traps, the SIGTERM signal is ignored.
• In the absence of traps, SIGINT is caught and handled. Thus, typing Ctrl+C, for example, will not
quit your interactive shell.
• Sending SIGHUP signals to all jobs on exit is configured with the huponexit option.
• Commands are executed upon read.
• Bash checks for mail periodically.
• Bash can be configured to exit when it encounters unreferenced variables. In interactive mode this
behavior is disabled.
• When shell built-in commands encounter redirection errors, this will not cause the shell to exit.
• Special built-ins returning errors when used in POSIX mode don't cause the shell to exit. The built-in
commands are listed in Section 1.3.2.
• Failure of exec will not exit the shell.
• Parser syntax errors don't cause the shell to exit.
• Simple spell check for the arguments to the cd built-in is enabled by default.
• Automatic exit after the length of time specified in the TMOUT variable has passed, is enabled.

---
id: First time Git Setup
aliases:
  - First time Git Setup
tags: []
---

# First time Git Setup

Git comes with a tool called `git config` that lets you get and set configuration
variables that control all aspects of how Git looks and operates. These variables
are stored in three different places:

1. [path]/etc/gitconfig file: Contains values applied to every user on the system and all their
repositories. If you pass the option --system to git config, it reads and writes from this file
specifically. Because this is a system configuration file, you would need administrative or
superuser privilege to make changes to it.
2. ~/.gitconfig or ~/.config/git/config file: Values specific personally to you, the user. You can
make Git read and write to this file specifically by passing the --global option, and this affects
all of the repositories you work with on your system.
3. config file in the Git directory (that is, .git/config) of whatever repository you’re currently
using: Specific to that single repository. You can force Git to read from and write to this file with
the --local option, but that is in fact the default. Unsurprisingly, you need to be located
somewhere in a Git repository for this option to work properly.

- Each level overrides values in the previous level, so values in `.git/config` 
trump those in `[path]/etc/gitconfig`.

You can view all of your settings and where they are comming from using:

```bash
git config --list --show-origin
```

## Your Identity

The first thing you should do when you install Git is to set your name and email.
This is important because every Git commit uses this info, and it's immutably 
baked into the commits you start creating:

```bash
git config --global user.name "Your Name"
git config --global user.email "Your Email"
```

### Your Editor

```bash
git config --global core.editor nvim
```

You can also check what Git thinks a specific key's value is by typing
`git config <key>`:

```bash
git config user.name
Your Name
```
If you have an unexpected value for any of these values you can query Git as to 
the origin for that value:

```bash
git config --show-origin rerere.autoUpdate
file:/home/johndoe/.gitconfig false
```

#### Getting Help

If you ever need help while using Git, there are three equivalent ways to get
the comprehensive manual page (manpage) help for any of the Git commands:

```bash
git help <verb>
git <verb> --Help
man git-<verb>
```

For example to get help with `git config` run:

```bash
git help config
```
If the manpages and this book aren’t enough and you need in-person help, you can
try the #git, #github, or #gitlab channels on the Libera Chat IRC server, which
can be found at https://libera.chat/. These channels are regularly filled with hundreds
of people who are all very knowledgeable about Git and are often willing to help.

For a quick refresher on the available options for a Git command, and a more 
concise output run:

```bash
git add -h
```

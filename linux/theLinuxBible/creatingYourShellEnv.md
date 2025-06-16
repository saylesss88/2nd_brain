---
id: creatingYourShellEnv
aliases:
  - Configuring your Shell
tags: []
---

# Configuring your Shell

Several config files support how your shell behaves.

- Some are executed for every user and every shell, while others are specific to
  the user who creates the config file.

   - To change the `etc/profile` or `etc/bashrc` files, you must be root.
   - It is better to create an `/etc/profile.d` file to add system-wide settings
     instead of editing those files directly.

## Bash Configuration Files

- `/etc/profile`  : System-wide configuration files, executed when you first
    log in. The file provides path and settings for env variables. Lasly,
    `/etc/profile` gathers shell settings from config files in `/etc/profile.d`.

- `/etc/bashrc`  : System-wide configuration files, executes for every user who
  runs the bash shell each time a bash shell is opened.

- `~/.
bash_profile` : This is used by each user to enter info that is specific to
their use of the shell. It is executed only once when the user logs in. This is
a good place to set env variables because once set, they are inherited by future
shells.

- `~/.bashrc` : This contains the info that is specific to your bash shells. It
  is read when you log in and also each time you open a new shell. This is the
  best location to add aliases so that your shell picks them up.

- `~/.bash_logout` : This is executed when you log out.

[[settingYourPrompt.md]]

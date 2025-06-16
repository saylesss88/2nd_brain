---
title: Environment Variables
date: 2024-10-28
tags: [bash, tag2]
---

# Environment Variables

Environment Variables allow you to store data in memory that can be easily
accessed by any program or script running from the shell.

- It's a handy way to store needed persistent data.

2 types

- Global Variables

- Local Variables

## Looking at global Environment Variables

Global environment variables are visible from the shell session and from any
spawned child subshells.

Local variables are available only in the shell that creates them.

- This makes global environment variables useful in applications that create
  child subshells, which require parent shell information.

- System environment variables almost always use all capital letters to
  differentiate them from normal user environment variables.

To view global environment variables, use the `env` or the `printenv` command:

```bash
printenv
```

Output:

```bash
BUN_INSTALL=/home/jr/.bun
COLORTERM=truecolor
DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus
DEBUGINFOD_URLS=https://debuginfod.archlinux.org
DESKTOP_SESSION=hyprland
DISPLAY=:1
EDITOR=nvim
FZF_CTRL_T_OPTS=
  --walker-skip .git,node_modules,target
  --preview 'bat -n --color=always {}'
  --bind 'ctrl-/:change-preview-window(down|hidden|)'
GDK_SCALE=1
GOPATH=/home/jr/go
GOROOT=/home/jr/.go
HL_INITIAL_WORKSPACE_TOKEN=28e05231-102f-4101-8c8c-6dd5c9222d51
HOME=/home/jr
HYPRLAND_CMD=Hyprland
HYPRLAND_INSTANCE_SIGNATURE=4520b30d498daca8079365bdb909a8dea38e8d55_1730135209_1316593605
LANG=en_US.UTF-8
LOGNAME=jr
MAIL=/var/spool/mail/jr
MANPAGER=nvim +Man!
MANROFFOPT=-c
MOTD_SHOWN=pam
MOZ_ENABLE_WAYLAND=1
OMF_CONFIG=/home/jr/.config/omf
OMF_PATH=/home/jr/.local/share/omf
PATH=/home/jr/go/bin:/home/jr/.local/bin:/home/jr/.bun/bin:/home/jr/.pyenv/shims:/home/jr/go/bin:/home/jr/.local/bin:/home/jr/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/home/jr/.local/share/flatpak/exports/bin:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/jr/.local/share/bin
PWD=/home/jr/notes/2nd_brain/linux
PYENV_ROOT=/home/jr/.pyenv
PYENV_SHELL=fish
QT_AUTO_SCREEN_SCALE_FACTOR=1
QT_QPA_PLATFORM=wayland;xcb
QT_QPA_PLATFORMTHEME=qt6ct
QT_WAYLAND_DISABLE_WINDOWDECORATION=1
SHELL=/usr/bin/fish
SHLVL=2
SSH_AUTH_SOCK=/run/user/1000/wezterm/agent.10978
SSL_CERT_DIR=/etc/ssl/certs
SSL_CERT_FILE=/etc/ssl/cert.pem
TERM=xterm-256color
TERM_PROGRAM=WezTerm
TERM_PROGRAM_VERSION=20241015-083151-9ddca7bd
USER=jr
WAYLAND_DISPLAY=wayland-1
WEZTERM_CONFIG_DIR=/home/jr/.config/wezterm
WEZTERM_CONFIG_FILE=/home/jr/.config/wezterm/wezterm.lua
WEZTERM_EXECUTABLE=/usr/bin/wezterm-gui
WEZTERM_EXECUTABLE_DIR=/usr/bin
WEZTERM_PANE=16
WEZTERM_UNIX_SOCKET=/run/user/1000/wezterm/gui-sock-10978
XDG_BACKEND=wayland
XDG_CURRENT_DESKTOP=Hyprland
XDG_DATA_DIRS=/home/jr/.local/share/flatpak/exports/share:/var/lib/flatpak/exports/share:/usr/local/share:/usr/share
XDG_RUNTIME_DIR=/run/user/1000
XDG_SEAT=seat0
XDG_SEAT_PATH=/org/freedesktop/DisplayManager/Seat0
XDG_SESSION_CLASS=user
XDG_SESSION_DESKTOP=Hyprland
XDG_SESSION_ID=2
XDG_SESSION_PATH=/org/freedesktop/DisplayManager/Session1
XDG_SESSION_TYPE=wayland
XDG_VTNR=1
_=/usr/bin/printenv
_JAVA_AWT_WM_NONREPARENTING=1
OLDPWD=/home/jr/notes/2nd_brain/linux
VISUAL=nvim
FZF_DEFAULT_COMMAND=rg --files --hidden --glob "!.git"
FZF_CTRL_T_COMMAND=rg --files --hidden --glob "!.git"
FZF_DEFAULT_OPTS=--height 60% --border sharp --layout reverse --color 'bg+:-1,fg:gray,fg+:white,border:black,spinner:0,hl:yellow,header:blue,info:green,pointer:red,marker:blue,prompt:gray,hl+:red' --prompt '∷ ' --pointer ▶ --marker ⇒
FZF_ALT_C_OPTS=--preview 'tree -C {} | head -n 10'
FZF_COMPLETION_DIR_COMMANDS=cd pushd rmdir tree ls
STARSHIP_SHELL=zsh
STARSHIP_SESSION_KEY=1384315391237812
BROWSER=zen-browser
MCFLY_FUZZY=true
MCFLY_RESULTS=20
MCFLY_INTERFACE_VIEW=BOTTOM
MCFLY_RESULTS_SORT=LAST_RUN
MCFLY_HISTFILE=/home/jr/.zsh_history
MCFLY_SESSION_ID=7sQ8mV46Pn00PTomRjafgpbJ
MCFLY_HISTORY=/tmp/mcfly.ImjkMk8y
MCFLY_HISTORY_FORMAT=zsh
```

### Looking at local environment variables

Local environment variables can only be seen in the local process in which they
are defined.

The `set` command displays all variables defined for a specific process,
including both local and global environment variables and user-defined
variables.

[[userDefinedVariables.md]]

## Setting global environment variables

Global environment variables are visible from any child processes created by the
parent process that sets the variable.

- The method used to create a global env variable is to first create a local
  variable and then export it to the global environment.

This is done by using the `export` command and the variable name minus the
dollar sign:

```bash
my_variable="I am Global now"

export my_variable

echo $my_variable
I am Global now

exit

echo $my_variable
I am Global now
```

Changing a global environment variable within a child shell does not affect the
variable's value in the parent shell:

- A child shell cannot even use the `export` command to change the parent
  shell's global environment variable's value.

#### Removing Environment Variables

```bash
echo $my_variable
I am Global now

unset my_variable

echo $my_variable
```

> [!TIP] Remember if you're doing anything _with_ the variable, use the dollar
> sign. If you're doing anything _to_ the environment variable, don't use the
> dollar sign. The exception to the rule is using `printenv`.

- If you're in a child process and `unset` an environment variable, it applies
  only to the child process.

[[defaultShellEnvVariables.md]]

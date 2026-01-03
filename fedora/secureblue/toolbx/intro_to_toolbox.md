# Intro to toolbox

**Toolbx** is a tool which allows the use of interactive command line
environments for software development and troubleshooting the host OS. It's
built on top of Podman and other standard container technologies from OCI.

Toolbx makes it easy to use a containerized environment for everyday software
development and debugging.

Particularly useful for OSTree based operating systems, The intention of these
systems is to discourage installation of software on the host, and instead
install software as (or in) containers. They mostly don't even have package
managers like DNF or YUM. This makes it difficult to set up a dev env or
troubleshoot the OS in the usual way.

Containers are created from stripped down versions of distro images.

Each toolbx container is an environment that you can enter from the command
line. Inside each one, you will find:

- Your existing username and permissions

- Access to your home dir and several other locations

- Access to both system and session D-Bus, system journal and Kerberos

- Common command line tools, including a package manager (DNF for fedora)

By connecting all this info, toolbx containers lose a certain amount of security
gained by using the containers technology. So you shouldn't treat toolbx
containers as a sandbox where you can execute any script you would never run on
any other system.

Create a toolbox:

```bash
toolbx create
```

The container is created with `podman create`, and its entry point is set to
`toolbox init-container`

By default, a Toolbx container is named after its corresponding image.

```bash
toolbox enter
```

A Toolbx container is an OCI container. Therefore, `toolbox enter` is analogous
to a `podman start` followed by a `podman exec`.

Create multiple toolboxes:

```bash
toolbox create -c toolbx-name
toolbox enter -c toolbx-name
```

## What is Shared Between Host and Container

Toolbox shares extensively with the host:

**Automatically mounted**:

- `$HOME` directory (entire home, read-write)

- Wayland/X11 sockets (GUI apps)

- D-Bus (system integration)

- `/dev` and udev database (hardware access)

- SSH agent

- systemd journal

- Network stack (including Avahi)

- Removable devices (USB drives)

- ulimits and resource limits

**NOT shared**:

- Root filesystem (`/` is container-specific)

- System packages (DNF installs stay in container)

- `/usr`, `/bin`, `/lib` (container has its own)

Files installed via user package managers (pip, cargo, npm) in `~/.local` or
`~/.cargo` **are shared** because they're in `$HOME`.

**Weird behavior with zsh**

To change default shells from bash to zsh I had to add the following to my
.bashrc:

```bash
if [ -x /usr/bin/zsh ]; then
    export SHELL=/usr/bin/zsh
    exec /usr/bin/zsh -l
fi
```

And the following to my .zshrc to make it stop rendering weird:

```zsh
export TERM=xterm-256color
unset LINES
unset COLUMNS

# Force prompt redraw
precmd() {
  zle && zle -R
}
```

## CONTAINERFILE

The **Containerfile** is a config file that automates the steps of creating a
container image. Container engines (Podman, Buildah, Docker) read instructions
from the Containerfile to automate the steps otherwise performed manually to
create an image.

## Set up a different Distro

```bash
toolbox create --distro arch
```

### Resources

- [Thrix Nix Toolbx](https://github.com/thrix/nix-toolbox)

- [Fedora Silverblue Toolbox](https://docs.fedoraproject.org/en-US/fedora-silverblue/toolbox/)

- [Open Container Initiative (OCI)](https://opencontainers.org/)

- [Getting started with podman](https://docs.podman.io/en/latest/)

- [Introduction to podman](https://docs.podman.io/en/latest/Introduction.html)

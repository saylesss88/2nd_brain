# Intro to toolbox

Toolbx makes it easy to use a containerized environment for everyday software
development and debugging.

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

```bash
toolbox enter
```

Create multiple toolboxes:

```bash
toolbox create -c toolbx-name
toolbox enter -c toolbx-name
```

### Resources

- [Fedora Silverblue Toolbox](https://docs.fedoraproject.org/en-US/fedora-silverblue/toolbox/)

- [Open Container Initiative (OCI)](https://opencontainers.org/)

- [Getting started with podman](https://docs.podman.io/en/latest/)

- [Introduction to podman](https://docs.podman.io/en/latest/Introduction.html)

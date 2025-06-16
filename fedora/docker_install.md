---
id: docker_install
aliases:
  - Docker Engine
tags: []
---

# Docker Engine

#docker

Docker Engine is a command-line interface (CLI) tool for working with containers. Docker Desktop provides a graphical user interface (GUI) on top of Docker Engine, making it easier to manage containers.

Here's how to install Docker Engine on Fedora:

    Update your system and install dnf-plugins-core:

```bash
sudo dnf update
sudo dnf install dnf-plugins-core
```

    Add the Docker repository:

```bash
sudo dnf config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo
```

    Install Docker Engine, containerd, and Docker Compose:

```bash
sudo dnf install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
```

    Start Docker:

```bash
sudo systemctl start docker
```

    Verify the installation:

```bash
sudo docker run hello-world
```

This command should download and run a test image, printing a confirmation message.

For Docker Desktop installation, you'd download the .rpm file from the Docker website and install it using dnf. You can find the specific instructions on the Docker documentation https://docs.docker.com/engine/install/fedora/.

# Intro to podman

Containers simplify production, distribution, discoverability, and usage of
applications with all of their dependencies and default configuration files.

Here's how to find our first Container Image:

```bash
podman search docker.io/busybox
```

The above command returns a list of publicly available container images on
DockerHub.

To run the busybox container image, it's just a single command:

```bash
podman run -it docker.io/library/busybox
```

Output:

```bash
/ #
```

You can mess with it a bit, but you'll find that running a small container with
few Linux utilities in it provides limited value, so exit out:

```bash
exit
```

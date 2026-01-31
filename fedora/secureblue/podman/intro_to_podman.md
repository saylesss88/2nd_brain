- secureblue sets Podman's container policy to **reject by default**, with only
  a small allowlist (often requiring signature verification), so pulls commonly
  fail with "Source image rejected ... rejected by policy".

- `rust:1.85-slim` is a short name Podman resolves to Docker Hub, so you need to
  explicitly trust that scope or switch to an allowed registry.

```bash
run0 podman image trust set -t accept docker.io/library/rust
```

Trust Debian:

```bash
# Allow specifically debian images
run0 podman image trust set -t accept docker.io/library/debian
# OR Allow all images from docker.io
run0 podman image trust set -t accept docker.io
```

Inspect `/etc/containers/policy.json` to see exactly what is changing.

If you are just testing and want to bypass the policy temporarily for this one
build (though podman build flags for this are limited compared to pull):

```bash
# Pull the image first with skipped verification, then build
podman pull --tls-verify=false docker.io/library/debian:stable-slim
podman build --no-cache -t mdbook-nix-repl .
```

(Note: --tls-verify=false sometimes bypasses signature policy depending on the
specific policy.json configuration, but image trust set is the correct fix.)

---

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

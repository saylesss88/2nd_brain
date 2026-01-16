## Creating a fedora distrobox

Make prompt show the container name:

```bash
# .zshrc
if [[ -f /run/.containerenv ]]; then
  box="${CONTAINER_ID:-${DBX_CONTAINER_NAME:-$HOSTNAME}}"
  PROMPT="[$box] %n@%m %~ %# "
fi
```

Trust the image

```bash
run0 podman image trust set -t accept registry.fedoraproject.org/fedora
```

```bash
distrobox create --name fedora-box --image registry.fedoraproject.org/fedora:latest --pull --yes
```

```bash
podman image trust show
```

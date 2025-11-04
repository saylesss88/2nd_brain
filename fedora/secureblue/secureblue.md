## Secureblue

- `/usr` is mounted read-only

- Settings can easily be overridden with changes in `/etc`, which is not
  read-only.

- [Secureblue DeepWiki](https://deepwiki.com/secureblue/secureblue/1-overview)

## Installing software

1. Check if it's already installed with `rpm -qa | grep x`

2. For GUI packages, use flatpak

3. For CLI packages, use `brew install`

- [Homebrew Formulaes](https://formulae.brew.sh/)

4. If a package isn't available via the above options, or if a package requires
   greater system integration, use `rpm-ostree install`.

- Add the unfiltered Flathub repo with `ujust enable-flathub-unfiltered`.

## Font configuration

Either use `rpm-ostree install` (if they're packaged as an RPM) or add them to
your users local font directory at `$HOME/.local/share/fonts`

## Containers

Software such as Podman and Distrobox need to be able to create user namespaces
to work without root. The privilege to do so is disabled by default in
secureblue, you can grant it by running:

```bash
ujust toggle-container-domain-userns-creation
```

## Sandboxing

The following command will toggle the ability of processes in the unconfined
SELinux domain to create user namespaces. It’s necessary for any apps that
require this feature, such as: browsers other than Trivalent, many Electron
apps, and bubblejail.

```bash
ujust toggle-unconfined-domain-userns-creation
```

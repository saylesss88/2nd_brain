## Flatpak tips

Flatpak is an application sandboxing system, flatpaks should be scoped down by
default to as few permissions as they need to function. You can use Flatseal to
see which permissions are enabled by default and report unnecessary ones by
submitting issues and/or PRs.

Add a repository:

```bash
flatpak remote-add name location
# Add the official repo with a per-user config
flatpak remote-add --if-not-exists --user flathub https://dl.flathub.org/repo/flathub.flatpakrepo
# Delete a repo
flatpak remote-delete name
```

List all added repositories:

```bash
flatpak remotes
```

Install a package for just your user:

```bash
flatpak install --user package-name
```

List available runtimes and applications:

```bash
flatpak remote-ls remote
```

Uninstall a runtime or application:

```bash
flatpak uninstall app
# To delete app data from ~/.var/app and from the permission store:
flatpak uninstall --delete-data app
```

## Sandbox permissions

Flatpak apps come with predefined sandbox rules which define the resources and
file system paths the app is allowed to access. To view the specific apps
permissions use:

```bash
flatpak info --show-permissions app
```

Override sandbox permissions of an app

```bash
flatpak override --nofilesystem=home app
```

Reset permissions back to their defaults with:

```bash
flatpak override --reset name
```

## Flatpak doesn't run on the linux-hardened kernel

The linux-hardened kernel sets `kernel.unprivileged_userns_clone` to `0`, so
only privileged users can create new user namespaces.

A simple fix could be to create a `/etc/sysctl.d/flatpak.conf`:

```conf
kernel.unprivileged_userns_clone=1
```

## Secureblue specific ujust commands

You can remove all permissions by default with the command:

```bash
ujust flatpak-permissions-lockdown
```

You will then have to explicitly enable any permissions you think are necessary
for the app.

You can revert with:

```bash
ujust flatpak-reset-global-overrides
# re-enable hardened_malloc for flatpak (the default)
ujust harden-flatpak
```

From a technical perspective, Flatpak does not require elevated privileges to
install apps, isolates apps from one another, and limits app access to the host
environment. It makes deep use of existing Linux security technologies such as
cgroups, namespaces, bind mounts, and seccomp as well as Bubblewrap for
sandboxing.

Use `bubblejail` if there's no Flatpak available for an app that you want to
sandbox. This requires
[enabling unconfined user namespaces](https://secureblue.dev/faq#unconfined-userns)
which is a security degradation.

## Runtimes

Runtimes provide basic dependencies used by applications. Each application must
be built against a runtime.

### Automatic Updates

Create `/etc/systemd/system/flatpak-update.service`:

```.service
[Unit]
Description=Update Flatpak
After=network-online.target
Wants=network-online.target

[Service]
Type=oneshot
ExecStart=/usr/bin/flatpak update --noninteractive --assumeyes

[Install]
WantedBy=multi-user.target
```

```bash
run0 systemctl daemon-reload
run0 systemctl enable flatpak-update.timer
run0 systemctl start flatpak-update.timer
```

🤤

## Resources

- [When Flatpaks sandbox cracks](https://www.linuxjournal.com/content/when-flatpaks-sandbox-cracks-real-life-security-issues-beyond-ideal)

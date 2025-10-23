## Pin your current Image and try another

1. To keep the default deployment:

```bash
sudo ostree admin pin 0
```

- `0` refers to the first deployment listed by `rpm-ostree status`

2. Verify that you have pinned your deployment of choice:

```bash
rpm-ostree status
```

3. After the deployment is pinned, you can upgrade your system by using the
   instructions found
   [here](https://docs.fedoraproject.org/en-US/fedora-silverblue/updates-upgrades-rollbacks/#upgrading)

4. Make sure you're running the latest update for the current version:

```bash
rpm-ostree upgrade
```

Reboot as needed

5. Print the available branches for Fedora Silverblue:

```bash
ostree remote refs fedora | grep silverblue | grep $(uname -m)
```

6. For example, to upgrade from 41 to 42:

```bash
rpm-ostree rebase fedora:fedora/42/x86_64/silverblue
```

> The default remote for Fedora Silverblue 42 is `fedora`. If this isn't the
> case, find out the remote name with `ostree remote list`.

You can rebase another Atomic Desktop, for example, Fedora Kinoite for KDE
variant.

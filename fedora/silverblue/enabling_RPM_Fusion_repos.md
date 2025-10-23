# Enabling RPM Fusion repos

non-free software is made available via the RPM fusion repos. This is
third-party software sources not officially affiliated with or endorsed by the
Fedora Project.

## First Installation

The first time you install RPM Fusion repos, you need to install the versioned
RPMs:

```bash
sudo rpm-ostree install https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm && reboot
```

## Major Fedora Updates

Once you have rebooted into the new deployment, you can run the following
command to remove the "lock" on the versioned packages that were installed
previously. This will enable the RPM Fusion repos to be automatically updated
and versioned correctly across major Fedora version rebases:

```bash
sudo rpm-ostree update --uninstall rpmfusion-free-release --uninstall rpmfusion-nonfree-release --install rpmfusion-free-release --install rpmfusion-nonfree-release && reboot
```

---
id: VirtualBox
aliases:
  - Virtual Box Install Guide
tags: []
---

# Virtual Box Install Guide

**Step 1:** Install VirtualBox Dependencies

```bash
sudo dnf install @development-tools
sudo dnf install kernel-headers kernel-devel dkms
```

**Step 2:** Add VirtualBox Repo to Fedora

Using your preferred terminal text editor, create the “/etc/yum.repos.d/virtualbox.repo” file:

```bash
sudo nvim /etc/yum.repos.d/virtualbox.repo
```

Then, put the content provided below in it, save it, and exit the file.

```bash
[virtualbox]
name=Fedora $releasever - $basearch - VirtualBox
baseurl=http://download.virtualbox.org/virtualbox/rpm/fedora/$releasever/$basearch
enabled=1
gpgcheck=1
repo_gpgcheck=1
gpgkey=https://www.virtualbox.org/download/oracle_vbox_2016.asc
```

Next, refresh the package list. When prompted, agree to import the GPG key signing packages from Oracle’s official VirtualBox repository by typing “y.”

```bash
sudo dnf update
```

**Step 3:** Install VirtualBox

```bash
sudo dnf install VirtualBox-7.0
```

**Step 4:** Install VirtualBox Extension Pack

To verify the exact version of the installed locally VirtualBox, you can use `vboxmanage`, a build-in VirtualBox’s command:

```bash
vboxmanage -v | cut -dr -f1
```

7.0.18

Use wget to downloat the VirtualBox Extension Pack of your version.

```bash
wget https://download.virtualbox.org/virtualbox/7.0.18/Oracle_VM_VirtualBox_Extension_Pack-7.0.18.vbox-extpack
```

Next, to install it with `vboxmanage` as follows:

```bash
sudo vboxmanage extpack install Oracle_VM_VirtualBox_Extension_Pack-7.0.18.vbox-extpack
```

You will be prompted to agree to Oracle’s license terms and conditions. To confirm, type “y” and press “Enter.”

Additionally, you can verify installed VirtualBox’s extension pack version by running the following:

```bash
vboxmanage list extpacks
```

**Step 5:** Add User to vboxusers Group

```bash
sudo usermod -a -G vboxusers $USER
```

Now, reboot and then check that you're in the vboxusers group:

```bash
groups $USER
```

## How to Uninstall VirtualBox

```bash
sudo dnf remove VirtualBox-7.0
```

Then you have two choices: disable the VirtualBox repository on your Fedora system:

```bash
sudo dnf config-manager --set-disabled virtualbox
```

Or completely remove it by deleting the repo's file:

```bash
sudo rm /etc/yum.repos.d/virtualbox.repo
```

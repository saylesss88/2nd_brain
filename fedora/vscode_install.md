---
id: vscode_install
aliases: []
tags: []
---

# How to install VSCode on Fedora

- Add the repo with:

```bash
sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc
echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" | sudo tee /etc/yum.repos.d/vscode.repo > /dev/null
```

- Install VSCode:

```bash
dnf check-update
sudo dnf install code # or code-insiders
```

- [!] Warning

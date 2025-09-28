---
id: openssh
aliases:
  - OpenSSH
tags: []
---

# OpenSSH

1.  Port Forwarding Rules: Configure Port Forwarding (for NAT):

    Open VirtualBox, go to your VM's Settings > Network > Adapter 1 (or the adapter you are using).

    Click on Advanced > Port Forwarding.

    Add a new rule with the following details:

        Name: SSH

        Protocol: TCP

        Host IP: Leave blank or put 127.0.0.1

        Host Port: 2222 (or any port you prefer)

        Guest IP: Leave blank

        Guest Port: 22

2.  SSH Keys:

Use `ssh-keygen` to generate an SSH key pair.

copy it over to the guest system

```bash
ssh-copy-id -p 2222 jr@localhost
```

3. Connect:

```bash
ssh -p 2222 jr@localhost
```

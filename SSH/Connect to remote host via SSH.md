---
id: Connect to remote host via SSH
aliases:
  - Connect to remote host via SSH
tags: []
---

# Connect to remote host via SSH

If the private key and public key are in the right places, then you can connect
to the system in this way:

```shell
ssh [username]@hostname
```

Where the username should be a valid user on the remote system and hostname is
DNS-reconizable or an IP address so that ssh can contact the remote system and
request for connection.

For example, to connect to the system named "linuxhandbook" with the username
"jr" use:

```shell
ssh jr@linuxhandbook
```

As explained before, the above command uses the private key on the local system
and public key on the remote system and verifies these are valid pairs. It
allows login if and only if key pair is valid and spawns a shell (type depends
on the configuration for the user on the remote system) for your use. You can
use the remote system as you are using the local system.

Suppose the private key is not added to the key agent, then you can do ssh login
as follows:

```shell
ssh -i /path/to/private/key/file username@hostname
```

[[Copying files between client and remote systems.md]]

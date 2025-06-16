---
id: whatIsSSH
aliases:
  - What is SSH?
tags: []
---

# What is SSH?

The `ssh` or secure shell is a network protocol for operating networking
services securely over a network. It uses encryption standards to securely
connect and lofin to the remote system.

- It stores a public key in the remote system's keychain.
- And a private key in the local system's keychain.

## Generate SSH key

```shell
ssh-keygen
```

By default, the ssh keys are stored in .ssh directory under your home directory.

If the key-location is DIR_PATH/keypairforssh, there will be 2 files:

1. DIR_PATH/keypairforssh
2. DIR_PATH/keypairforssh.pub

1 is the private key file which you must not share with anyone.
2 is the public key file which you can share with remote systems (by means of
other trusted communication such as mail, physical transfer, and other secured
communication tools)

### Add private key to the key-agent

When the key pair is created, it just exists as a set of two files. In order to
connect to the remote system, it has to use the private key. So one should
inform that DIR_PATH/keypairforssh is the location of the private key.

This is done by:

```shell
ssh-add key-location
```
In our case, it is 

```shell
ssh-add DIR_PATH/keypairforssh
```
[[Connect to remote host via SSH.md]]

---
title: Kernel Module
date: 2024-11-28
tags: [linux, kernel]
---

# Kernel Module

Kernel modules are pieces of code that can be loaded into the kernel upon
demand. They extend the functionality of the kernel without the need to reboot
the system.

- A module can be configured as a built-in or loadable.

## Obtaining Information

Modules are stored in the `/lib/modules/kernel_release` directory. You can use
the `uname -r` command to get your current kernel release version.

To show what kernel modules are currently loaded:

```bash
lsmod
```

To show info about a module:

```bash
modinfo module_name
```

To list the options that are set for a loaded module use `systool(1)` from
`sysfsutils`:

```bash
systool -v -m module_name
```

To display the comprehensive configuration of all the modules:

```bash
modprobe -c | less
```

To display the configuration of a specific module:

```bash
modprobe -c | grep module_name
```

List the dependencies of a module (or alias), including the module itself:

```bash
modprobe --show-depends module_name
```

## Manual Module Handling

Kernel modules are handled by tools provided by `kmod` package.

To load a module:
`sudo modprobe module_name`

To load a module by filename (i.e. one that's not installed in
`/usr/lib/modules/$(uname -r)/`):

```bash
sudo insmod filename [args]
```

To unload a module:

```bash
sudo modprobe -r module_name
```

Or, alternatively:

```bash
sudo rmmod module_name
```

### Setting Module Options

Manually at load time using modprobe:

Parameters are specified on command line using simple `key=value` assignments:

```bash
sudo modprobe module_name parameter_name=parameter_value
```

Using files in `/etc/modprobe.d/`:

Files in /etc/modprobe.d/ directory can be used to pass module settings to udev, which will use modprobe to manage the loading of the modules during system boot. Configuration files in this directory can have any name, given that they end with the .conf extension. The syntax is:

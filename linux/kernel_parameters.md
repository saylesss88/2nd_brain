---
id: kernel_parameters
aliases:
  - Kernel Parameters
tags: []
---

# Kernel Parameters

When the linux kernel starts, it receives a set of text-based *kernel parameters*
containing a few additional system details.

   - The parameters specify many different types of behavior, like the amount
   diagnostic output the kernel should produce and device driver-specific options.

You can view the parameters passed to your system's currently running kernel by
looking at the `/proc/cmdline` file:

```bash
cat /proc/cmdline
```

- The parameters are either simple one-word flags, such as `ro` and `quiet`,
  or `key=value` pairs, such as `vt.handoff=1`

- Many of the parameters are unimportant, like the `splash` flag for
displaying a splash screen.

- The `root` parameter (critical), is the location of the root filesystem;
without it, the kernel cannot properly perform the user space start.

The root filesystem can be specified as a device file, as in this example:

```bash
root=/dev/sda1
```

You may also see a `UUID` like this
-------------------------------------------------------------------------------

```bash
 root=UUID=db3c6f4b-b3f8-49d4-aeec-d5e93b3c1575
```

  - The `ro` parameter instructs the kernel to mount the root filesystem
  read-only mode upon user space start

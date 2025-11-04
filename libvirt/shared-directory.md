## Shared Directory

On the host, create the directory you want to share:

```bash
mkdir vm-shared
run0 chown jr:jr vm-shared
```

In virt-manager, open the VM configuration:

- Go to "Memory" and enable "shared memory"

- Click "Add Hardware" and select "Filesystem"

- Set the "Driver" to "virtiofs"

- Set the "Source Path" to the host folder `/home/jr/vm-shared`

- Set the "Target Path" to `host-shared`

In the VM, mount the shared folder with:

```bash
run0 mkdir /tmp
run0 mount -t virtiofs host-share /mnt
```

Add files to your Hosts ~/vm-shared directory and on the VM, the files will be
placed in `/mnt`.

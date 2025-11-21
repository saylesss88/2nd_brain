## Virtualization and Hypervisors Explained

**Virtualization**: Virtualization is a technology that allows you to create
virtual, simulated environments from a single, physical machine.:

**KVM (Kernel Virtual Machine)**: The actual hypervisor. It's a Linux kernel
module that allows the processor to run another OS (the guest) directly on the
hardware (CPU/RAM). It provides the speed of native execution. KVM is a type 1
hypervisor.

**QEMU (Quick EMUlator)**: The machine emulator. KVM handles the CPU, but QEMU
handles all the virtual hardware (disks, network cards, graphics). When used
with KVM, QEMU uses KVM's speed for the CPU, making it very fast. QEMU is a type
2 hypervisor.

**libvirt**: The management layer/API. It's a daemon and a set of libraries that
provides a common, stable interface to manage various hypervisors (QEMU/KVM,
Xen, etc.). It handles network setup, storage paths, and XML configuration.

**virt-manager**: The graphical user interface (GUI). It's a tool that
communicates with libvirt to let you easily create, start, stop, and manage your
virtual machines without dealing with complex command line arguments or XML.

Libvirt is a collection of software that provides a convenient way to manage
virtual machines and other virtualization functionality, such as storage and
network interface management.

These software pieces include an API library, a daemon (libvirtd), and a command
line utility (virsh).

## Virsh Usage

```bash
virsh -c qemu:///system list --all
```

## Performance

### Resources

- [kvm-qemu-libvirt-virtualization](https://bitgrounds.tech/posts/kvm-qemu-libvirt-virtualization/)

- [Virtualization and Hypervisors](https://sumit-ghosh.com/posts/virtualization-hypervisors-explaining-qemu-kvm-libvirt/)

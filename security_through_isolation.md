## Security through isolation

**Why**

Every piece of software has a vulnerability, eventually someone will find the
vulnerability and exploit it. Isolating software from the system is an effective
way to ensure that the system stays secure even if such a vulnerability is
exploited.

If vulnerable software is running in a VM and is then exploited, only that VM is
compromised and everything else, such as the host machine and other VMs are
safe. This approach is very useful in situations where you know you'll be
targeted.

## Virtual Machines

VMs run operating systems and software in an isolated virtual environment.
Hardware such as RAM, CPU, and GPU are also virtualized by the machine and can
be set to use only a portion of the host's hardware. VMs also make use of
virtual disks, usually these are hosted from a file stored on the host machine,
these virtual disks help keep Virtual Machines isolated from the host. Any
properly configured VM that is running the proper settings, on the proper host,
and running the proper guest OS is going to be very difficult fro things such as
malware to escape.

That said, VMs aren't indestructible, malware can fill the storage of the
virtual hard disk rendering it difficult to use. Virtual Machine escapes also
become easier if there are misconfigurations such as enabling sharing memory
with other Virtual Machines (KVM), or outdated virtualization software. Your VMs
should have the absolute minimum they need to run and be used properly. If they
don't need a virtual printer device, remove it. Users also shouldn't solely rely
on the Virtual Machine if the OS you're running as the host is not secure, then
a Virtual Machine will not protect you.

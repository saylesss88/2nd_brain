# Firewalld

Firewalld provides a dynamically managed firewall with support for
network/firewall zones to define the trust level of network connections or
interfaces.

A firewalld zone defines the level of trust for network connections, interfaces
and source addresses bound to the zone. The zone combines services, ports,
protocols, masquerading, port/packet forwarding, icmp filters and rich rules.
The zone can be bound to interfaces and source addresses.

`firewall-config` -> GUI app for firewalld.

D-BUS is a message bus system and inter-process communication (IPC) mechanism
commonly used on Linux and other Unix-like systems. Its main purpose is to allow
different programs (processes) running on the same machine to communicate with
each other in a standardized and reliable way.

**What is a zone?**

A network zone defines the level of trust for connections. This is a one to many
relation, which means that a connection can only be part of one zone, but a zone
can be used for many network connections.

To configure or add zones, you can use `firewall-cmd`, `firewall-config`, or the
D-BUS interface. Or you can create or copy a zone file in one of the
configuration directories. `@PREFIX/@/lib/firewalld/zones` is used for default
and fallback configurations and `/etc/firewalld/zones` is used for user created
and customized configuration files.

You can also use `nm-connection-editor` to change your Firewall zone.

**Predefined services**: A service is a combination of port and/or protocol
entries. Oprionally netfilter helper modules can be added and also IPv4 and IPv6
destination address.

There are two configuration modes, **runtime** & **permanent**. To modify the
firewall settings in permanent mode, use the `--permanent` option with the
`firewall-cmd` command.

```bash
run0 firewall-cmd --permanent <other options>
```

Without adding `--permanent`, the command modifies runtime mode. To change
settings in both modes, you can use two methods:

- Change runtime settings and then make them permanent:

1. Change the runtime settings:

```bash
firewall-cmd <other options>
```

2. Use `--runtime-to-permanent` to make the changes permanent.

```bash
firewall-cmd --runtime-to-permanent
```

- Set permanent settings and reload the settings into runtime mode:

1. Make the changes in permanent mode:

```bash
firewall-cmd --permanent <other options>
```

2. Reload the settings:

```bash
firewall-cmd --reload
```

## Controlling ports

What are ports?

Ports are logical devices that enable an operating system to receive and
distinguish network traffic and forward it accordingly to system services. These
are usually represented by a daemon that listens on the port, that is it waits
for any traffic coming to this port.

**Opening a port on the command-line**

1. Get a list of allowed ports in the current zone:

```bash
firewall-cmd --list-ports
```

2. Add a port to the allowed ports to open it for incoming traffic:

```bash
run0 firewall-cmd --add-port=port-number/port-type
```

3. Make the new settings persistent:

```bash
run0 firewall-cmd --runtime-to-permanent
```

The port types are either tcp, udp, sctp, or dccp. The type must match the type
of network communication.

**Closing a port**

```bash
run0 firewall-cmd --remove-port=port-number/port-type
```

```bash
run0 firewall-cmd --runtime-to-persistent
```

- [Control of System Accessibility by firewalld](https://docs.fedoraproject.org/en-US/quick-docs/firewalld/)

- [Firewalld Fedora Wiki](https://fedoraproject.org/wiki/Firewalld)

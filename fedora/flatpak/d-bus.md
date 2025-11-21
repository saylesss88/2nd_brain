# d-bus

- [maemo D-Bus](https://maemo.org/maemo_training_material/maemo4.x/html/maemo_Platform_Development_Chinook/Chapter_01_DBus_The_Message_Bus_System.html)

- [dbus user vs system session](https://forums.whonix.org/t/dbus-user-vs-system-session/10913)

- [tldp Interprocess Communication Mechanisms](https://tldp.org/LDP/tlk/ipc/ipc.html)

- [Understanding Unix Sockets](https://www.geeksforgeeks.org/linux-unix/understanding-unix-sockets/)

## Unix Sockets

A unix socket is a software endpoint facilitating bidirectional communication
between processes, regardless of their location within the system or even beyond
its borders.

Unix sockets offer two distinct flavors:

1. Network sockets: These are the long-distance runners, enabling communication
   across networks using protocols like TCP/IP.

2. Domain sockets: These facilitate communication between processes within the
   same system. Think of them as private pipes connecting programs within the
   Linux kingdom.

Each Unix socket comprises several crucial elements:

- **Domain**: This specifies the communication protocol, like `AF_INET` for
  `TCP/IP` or `AF_UNIX` for domain sockets.

- **Type**: This defines the communication style, like `SOCK_STREAM` for
  reliable byte streams or `SOCK_DGRAM` for unreliable datagrams.

- **File** Descriptor: This is the unique identifier assigned to the socket,
  used for accessing and manipulating it.

- **Address**: This identifies the socket, either as an IP address and port for
  network sockets or a path on the filesystem for domain sockets.

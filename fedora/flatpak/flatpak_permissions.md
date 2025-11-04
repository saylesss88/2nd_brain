## Flatpak Permissions with Flatseal

## D-Bus session bus VS. D-bus system bus

The difference between socket=session-bus and socket=system-bus in Flatpak
permissions relates to the two main types of D-Bus communication buses in Linux
systems:

**Session Bus**: This bus is user-specific and tied to the user’s graphical
login session. It facilitates communication between user-level applications
running within that session. Each user and each graphical session typically has
its own session bus instance. Apps that run in the user's graphical environment
and need to interact with other desktop applications, like file managers, media
players, or key managers (e.g., Kleopatra), use the session bus. Permissions
enabling socket=session-bus allow the Flatpak app to communicate with services
and applications in the user's session environment.

**System Bus**: This bus is system-wide and runs with elevated privileges
(usually as root). It is used for inter-process communication involving system
services and daemons that affect the whole system, such as hardware management,
power management, and system configuration. Apps that need to interact with
system-level services or hardware management interfaces require access to the
system bus. Enabling socket=system-bus grants the app permission to communicate
with system-level services.

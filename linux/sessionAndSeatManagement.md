---
id: sessionAndSeatManagement
aliases:
  - Session and Seat Management
tags:
  - linux
  - session management
---

# Session and Seat Management

Session and seat management isn't necessary for every setup, but it can be used
to safely create temporary runtime directories, provide access to hardware
devices and multi-seat capabilities, and control system shutdown.

## D-Bus

D-Bus is an IPC (Inter-Process Communication) mechanism used by userspace
software in Linux. D-Bus can provide a system bus and/or a session bus, the
latter being specific to a user session.

- It is a message bus system that allows processes to send messages to each
  other, even if they were not designed to communicate directly. This makes it a
  powerful tool for integrating different software components and creating complex
  applications.

- To provide a system bus, you should enable the `dbus` service. This might
  require a reboot to work properly.

- To provide a session bus, you can start a given program (usually a window
  manager or interactive shell) with `dbus-run-session`.

  - If a D-Bus session is active for the current session, the environment
    variable `DBUS_SESSION_BUS_ADDRESS` will be set.

`elogind`: A Standalone Session and Seat Manager

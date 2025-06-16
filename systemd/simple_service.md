---
id: Simple Service
aliases: [systemd]
---

# Simple Service

The following unit file creates a service that will execute /usr/sbin/foo-daemon
Since no Type= is specified, the default Type=simple will be assumed.

```text
[Unit]
Description=Foo
[Service]
ExecStart=/usr/sbin/foo-daemon
[Install]
WantedBy=multi-user.target
```
- Systemd assumes here that the process started by systemd will continue running
until the service terminates. If the program daemonizes itself (i.e. forks), use
`Type=forking` instead.

## Oneshot service

```text
[Unit]
Description=Cleanup old Foo data
[Service]
Type=oneshot
ExecStart=/usr/sbin/foo-cleanup
[Install]
WantedBy=multi-user.target
```

### Custom Dependency

```text
[Unit]
Requires=new dependency
After=new dependency
```

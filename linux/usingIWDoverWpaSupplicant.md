---
id: Using iwd instead of wpa supplicant
aliases:
  - Using iwd instead of wpa supplicant
tags: []
---

# Using iwd instead of wpa supplicant

Connman can use `iwd` to connect to wireless networks. As `connman` will start
`wpa_supplicant` when it finds it, it is recommended to uninstall
`wpa_supplicant`.

> [!Note] ConnMan is probably unnecessary for IWD users, as IWD can handle its
> own networking configuration, in which case connmand should me stopped.

- Currently the `-i` option of iwd seems to cause the WiFi-interface to be
  hidden from `connman`.

Create the following service file which should cause `connman` to use `iwd` to
connect to wireless networks, regardless if `wpa_supplicant` is installed:

```bash /etc/systemd/system/connman_iwd.service
[Unit]
Description=Connection service
DefaultDependencies=false
Conflicts=shutdown.target
RequiresMountsFor=/var/lib/connman
After=dbus.service network-pre.target systemd-sysusers.service iwd.service
Before=network.target multi-user.target shutdown.target
Wants=network.target
Requires=iwd.service

[Service]
Type=dbus
BusName=net.connman
Restart=on-failure
ExecStart=/usr/bin/connmand --wifi=iwd_agent -n
StandardOutput=null
CapabilityBoundingSet=CAP_NET_ADMIN CAP_NET_BIND_SERVICE CAP_NET_RAW CAP_SYS_TIME CAP_SYS_MODULE
ProtectHome=true
ProtectSystem=true

[Install]
WantedBy=multi-user.target
```

Then enable/start the `connman_iwd` service.

Advantage of using `iwd` instead of `wpa_supplicant` is, that the ping times
seem to be much more consistent and the connection seems to be more reliable.

## Settings

Settings and profiles are automatically created for networks the user connects to often. They contain fields for the passphrase, essid and other information. Profile settings are stored in directories under /var/lib/connman/ by their service name. To view all network profiles run: 

```bash
# cat /var/lib/connman/*/settings
```

### Technologies

Various hardware interfaces are referred to as technologies by connman.

To list available technologies run:

```bash
connmanctl technologies
```
To get just the types by their name one can use this one liner:

```bash
connmanctl technologies | awk '/Type/ { print $NF }'
```
> [!Note] The field Type = tech_name provides the technology by type. They can
> be toggled on and off with:

```bash
connmanctl enable technology_type
```

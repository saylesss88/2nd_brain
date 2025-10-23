## Install and Enable Firewalld

```bash
sudo pacman -S firewalld
sudo pacman -S ebtables
```

```bash
sudo systemctl enable firewalld
sudo systemctl start firewalld
```

Restart `libvirtd` (This is crucial for it to register with firewalld)

```bash
sudo systemctl restart libvirtd
```

## Zones

Firewalld uses a zone-based system to define different levels of trust. The rule
of thumb is to assign a network interface or source IP adress to the zone that
best reflects the trust level of that connection.

Ensure the `libvirtd` zone was added:

```bash
sudo firewall-cmd --get-zones
# Inspect the libvirt zone
sudo firewall-cmd --zone=libvirt --list-all
```

| Zone name | Trust Level       | Typical Use Case                                                                    |
| --------- | ----------------- | ----------------------------------------------------------------------------------- |
| drop      | Least Trusted     | Drops all incoming packets without a reply. Only outgoing connections are possible  |
| block     | Very Low Trust    | Rejects all incoming packets with an ICMP message.                                  |
| public    | Untrusted         | For use in public areas, Accepts explicitly allowed incoming connections            |
| external  | Untrusted/Gateway | For the external interface when the host is a gateway/router, with NAT masquerading |
| work      | Fairly Trusted    | For work environments. You mostly trust other computers.                            |
| home      | Fairly Trusted    | For home environments. You generally trust other computers.                         |
| internal  | Trusted           | For internal networks                                                               |
| trusted   | Most Trusted      | All network connections are accepted. Use with extreme caution                      |

**Deciding on the Zone**

1. Identify the Networks Purpose: Is the interface connecting to the internet
   (untrusted, use `public` or `external`), or a local home network (fairly
   trusted, use `home`), or a secure private server network (trusted, use
   `internal` or `trusted`)

2. Assign the Zone: Once you know the trust level, you assign the zone to a
   network interface (e.g., `eth0`) or a source IP/network range.

## Set Hostname in Arch

Edit `/etc/hostname` with your chosen hostname:

```bash
magic
```

Update your `/etc/hosts` file to match:

```hosts
# Static table lookup for hostnames.
# See hosts(5) for details.

# IPv4 localhost
127.0.0.1        localhost

# IPv6 localhost
::1              localhost ip6-localhost ip6-loopback

# System hostname
127.0.1.1        magic

# IPv6 multicast
ff02::1          ip6-allnodes
ff02::2          ip6-allrouters
```

Apply the changes:

```bash
sudo hostnamectl set-hostname magic
```

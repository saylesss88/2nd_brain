## libvirtd on Arch with dnsmasq & dnscrypt-proxy + firewall

```bash
ip link show
ip link show
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN mode DEFAULT group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
2: enp4s0: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc mq state DOWN mode DEFAULT group default qlen 1000
    link/ether 1c:83:41:41:1e:eb brd ff:ff:ff:ff:ff:ff
    altname enx1c8341411eeb
3: eno1: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc fq_codel state DOWN mode DEFAULT group default qlen 1000
    link/ether 1c:83:41:41:1e:ea brd ff:ff:ff:ff:ff:ff
    altname enp2s0
    altname enx1c8341411eea
4: wlp3s0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP mode DORMANT group default qlen 1000
    link/ether 40:9c:a7:62:27:bc brd ff:ff:ff:ff:ff:ff
    altname wlx409ca76227bc
5: virbr2: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc htb state DOWN mode DEFAULT group default qlen 1000
    link/ether 52:54:00:25:7d:00 brd ff:ff:ff:ff:ff:ff
6: virbr1: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc htb state DOWN mode DEFAULT group default qlen 1000
    link/ether 52:54:00:fa:63:1a brd ff:ff:ff:ff:ff:ff
```

```bash
sudo ufw allow in on virbr1
sudo ufw allow out on virbr1
sudo ufw allow in on virbr2
sudo ufw allow out on virbr2
```

```bash
sudo virsh net-list --all
```

An example of how to make a net inactive.

```bash
sudo virsh net-destroy default
```

```bash
sudo virsh net-start default
sudo virsh net-autostart default
```

default was inactive

Add an exception to `/etc/dnsmasq.conf` by adding the following lines:

```conf
except-interface=virbr0
bind-interfaces
```

The `bind-interfaces` option ensures `dnsmasq` only binds to explicitly
configured interfaces, and `except-interface=virbr0` excludes the
libvirt-managed interface.

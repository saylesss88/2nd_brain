## dnscrypt-proxy on Arch Linux

```bash
sudo pacman -S dnscrypt-proxy
```

- [Arch Wiki dnscrypt-proxy](https://wiki.archlinux.org/title/Dnscrypt-proxy)

Edit `/etc/dnscrypt-proxy/dnscrypt-proxy.toml` to add your chosen resolvers etc.

Modify `resolv.conf`:

```conf
#/etc/resolv.conf
nameserver ::1
nameserver 127.0.0.1
options edns0
```

Disable any services bound to port 53

```bash
ss -lp 'sport = :domain'
```

```bash
sudo systemctl stop systemd-resolved
sudo systemctl disable systemd-resolved
```

`libvirtd` is another problem application for this because it uses dnsmasq by
default.

## dnsmasq as a local DNS server

Edit the `listen_addresses` in `dnscrypt-proxy.toml`:

```toml
listen_addresses = ['127.0.0.1:5353', '[::1]:5353']
```

Edit `/etc/dnsmasq.conf`:

```conf
listen-address=127.0.0.1,::1
server=127.0.0.1#5353
server=::1#5353
```

```bash
sudo systemctl restart dnsmasq
sudo systemctl restart dnscrypt-proxy
```

**How it Works**

You've created a two-step process for your DNS queries. Instead of your computer
directly asking a DNS server for a website's IP address, it now sends the
request to dnsmasq first.

1. dnsmasq as the Local DNS Server: You configured dnsmasq to listen on
   127.0.0.1 (your local machine). This means that all DNS queries from your
   system, including those from libvirtd, are sent to dnsmasq.

2. dnsmasq Forwards to dnscrypt-proxy: You told dnsmasq to use dnscrypt-proxy as
   its upstream DNS server with server=127.0.0.1#5353. So, dnsmasq gets the DNS
   request and immediately forwards it to dnscrypt-proxy, which is listening on
   port 5353.

3. dnscrypt-proxy Encrypts and Resolves: dnscrypt-proxy then takes the forwarded
   request, encrypts it, and sends it to a secure DNS resolver on the internet.
   It receives the IP address back, decrypts it, and sends it back to dnsmasq.

4. dnsmasq Returns the Answer: Finally, dnsmasq receives the IP address from
   dnscrypt-proxy and sends it back to the program that made the original
   request (e.g., your web browser, or in this case, a virtual machine running
   under libvirtd).

Enabling `libvirtd` caused ERRORS:

```bash
 sudo systemctl status libvirtd
● libvirtd.service - libvirt legacy monolithic daemon
     Loaded: loaded (/usr/lib/systemd/system/libvirtd.service; enabled; preset: disabled)
     Active: active (running) since Sun 2025-09-21 19:58:53 EDT; 3s ago
 Invocation: 16bb5dab06754f6eb4d758b8c0f0f0c6
TriggeredBy: ● libvirtd.socket
             ● libvirtd-admin.socket
             ● libvirtd-ro.socket
       Docs: man:libvirtd(8)
             https://libvirt.org/
   Main PID: 21796 (libvirtd)
      Tasks: 21 (limit: 32768)
     Memory: 23.1M (peak: 23.9M)
        CPU: 824ms
     CGroup: /system.slice/libvirtd.service
             └─21796 /usr/bin/libvirtd --timeout 120

Sep 21 19:58:53 archlinux systemd[1]: Starting libvirt legacy monolithic daemon...
Sep 21 19:58:53 archlinux systemd[1]: Started libvirt legacy monolithic daemon.
Sep 21 19:58:53 archlinux dnsmasq[21880]: failed to create listening socket for 192.168.122.1: Address already in use
Sep 21 19:58:53 archlinux dnsmasq[21880]: FAILED to start up
Sep 21 19:58:53 archlinux libvirtd[21796]: libvirt version: 11.7.0
Sep 21 19:58:53 archlinux libvirtd[21796]: hostname: archlinux
Sep 21 19:58:53 archlinux libvirtd[21796]: internal error: Child process (VIR_BRIDGE_NAME=virbr0 /usr/bin/dnsmasq --co>
                                           dnsmasq: failed to create listening socket for 192.168.122.1: Address alrea>
Sep 21 19:58:53 archlinux libvirtd[21796]: Cannot get interface flags on 'virbr0': No such device
Sep 21 19:58:54 archlinux libvirtd[21796]: error destroying network device virbr0: No such device
[  8:00PM ]  [ jr@archlinux:/etc/dnscrypt-proxy ]
 $ sudo lsof -i :53
COMMAND     PID    USER  FD   TYPE DEVICE SIZE/OFF NODE NAME
librewolf  2219      jr 181u  IPv6  66455      0t0  UDP localhost:56569->localhost:domain
dnsmasq   18524 dnsmasq   4u  IPv4  62586      0t0  UDP *:domain
dnsmasq   18524 dnsmasq   5u  IPv4  62587      0t0  TCP *:domain (LISTEN)
dnsmasq   18524 dnsmasq   6u  IPv6  62588      0t0  UDP *:domain
dnsmasq   18524 dnsmasq   7u  IPv6  62589      0t0  TCP *:domain (LISTEN)
```

Restarting libvirtd seems to have fixed it!

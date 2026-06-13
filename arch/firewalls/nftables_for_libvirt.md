1. Allow forwarding through the virbrO bridge

```conf
#!/usr/bin/nft -f
flush ruleset

table inet filter {
    chain input {
        type filter hook input priority 0; policy drop;
        ct state established,related accept
        iif lo accept
        ip protocol icmp accept
        ip6 nexthdr icmpv6 accept
        ct state invalid drop
        tcp dport 22 accept
        iif virbr0 udp dport { 53, 67 } accept
        iif virbr0 tcp dport 53 accept
    }
    chain forward {
        type filter hook forward priority 0; policy drop;
        iif virbr0 accept
        oif virbr0 ct state established,related accept
    }
    chain output {
        type filter hook output priority 0; policy accept;
    }
}

table ip nat {
    chain postrouting {
        type nat hook postrouting priority 100;
        ip saddr 192.168.122.0/24 masquerade
    }
}
```

2. Enable IP forwarding

```bash
# /etc/sysctl.d/99-libvirt.conf
net.ipv4.ip_forward = 1
```

3. Tell libvirt not to manage its own firewall rules (avoids conflicts):

```bash
# /etc/libvirt/network.conf
firewall_backend = "nftables"
```

Or set `firewall_backend = "none"` if you want to manage everything yourself via
nftables as above.

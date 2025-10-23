```conf
#!/usr/sbin/nft -f

table inet filter {
    chain input {
        type filter hook input priority 0; policy drop;

        # Allow loopback
        iif "lo" accept

        # Accept established and related connections
        ct state established,related accept

        ip protocol icmp accept
        # Allow ICMPv6
        ip6 nexthdr icmpv6 accept

        # Allow SSH (port 22)
        tcp dport 22 ct state new accept

        # Allow HTTP and HTTPS (ports 80 and 443)
        tcp dport {80,443} ct state new accept
    }

    chain forward {
        type filter hook forward priority 0; policy drop;
    }

    chain output {
        type filter hook output priority 0; policy accept;

        # Allow DNS queries
        udp dport 53 accept
        tcp dport 53 accept
    }
}
```

Output:

```conf
# -------------------------------------------------------------------------
# OUTPUT CHAIN (Outgoing Traffic originating from this host)
# Default is to DROP all outgoing traffic for maximum security
# -------------------------------------------------------------------------
chain output {
    type filter hook output priority filter; policy drop; # <-- Set to DROP for security

    # Allow essential local communication
    oif "lo" accept

    # Allow replies for established and related connections (critical)
    ct state established,related accept

    # Allow all necessary outgoing ICMPv6 types, including MLDv2 Report (Type 143)
    ip6 nexthdr icmpv6 icmpv6 type {
        echo-request,            # Your own pings
        destination-unreachable,
        time-exceeded,
        parameter-problem,
        mld2-listener-report     # Type 143 (the fix)
    } accept

    # Allow DNS queries (UDP and TCP)
    udp dport 53 accept
    tcp dport 53 accept

    # Allow general outbound web access (e.g., for updates, API calls)
    tcp dport { 80, 443 } accept

    # Allow Git (and general SSH client) outgoing connections
    tcp dport 22 ct state new accept

    # Allow NTP (Network Time Protocol) for time synchronization
    udp dport 123 accept

    # Log packets that reach the end of the chain before they are dropped by the policy
    log prefix "nft-output-drop: "
}
```

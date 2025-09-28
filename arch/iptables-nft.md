I went with `iptables-nft` which enables you to use iptable syntax to configure
nftable rules. When you install `iptables-nft`, pacman will automatically remove
the iptables package.

```bash
sudo pacman -S iptables-nft
```

Flush existing rules:

> ⚠️ Warning: Flushing rules will remove all existing firewall configurations.
> Ensure no critical services are running, or review existing rules with
> `iptables -L -v -n` and `ip6tables -L -v -n` before proceeding.

```bash
sudo iptables -F
sudo iptables -X
sudo iptables -t nat -F
sudo iptables -t nat -X
```

Setup policies to DROP:

```bash
# Set default policy for IPv4
sudo iptables -P INPUT DROP
sudo iptables -P FORWARD DROP
sudo iptables -P OUTPUT ACCEPT

# Set default policy for IPv6 (optional, but good practice)
sudo ip6tables -P INPUT DROP
sudo ip6tables -P FORWARD DROP
sudo ip6tables -P OUTPUT ACCEPT
```

### Default Policy Explanations

- **INPUT DROP**: Blocks all incoming connections by default, enhancing
  security.

- **FORWARD DROP**: Prevents packet forwarding, suitable for non-router systems.

- **OUTPUT ACCEPT**: Allows all outgoing connections, typical for
  desktops/servers.

Allow Localhost Traffic:

```bash
sudo iptables -A INPUT -i lo -j ACCEPT
sudo ip6tables -A INPUT -i lo -j ACCEPT
```

Add ICMPv6 to avoid breaking IPv6 connectivity:

```bash
sudo ip6tables -A INPUT -p ipv6-icmp -j ACCEPT
```

Allow established/related connections:

```bash
sudo iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT
sudo ip6tables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT
```

Allow SSH (port 22) and HTTP/HTTPS (ports 80/443) **only if you use these**:

```bash
# Allow SSH (port 22)
sudo iptables -A INPUT -p tcp --dport 22 -j ACCEPT
sudo ip6tables -A INPUT -p tcp --dport 22 -j ACCEPT

# Allow HTTP/HTTPS (ports 80, 443)
sudo iptables -A INPUT -p tcp -m multiport --dports 80,443 -j ACCEPT
sudo ip6tables -A INPUT -p tcp -m multiport --dports 80,443 -j ACCEPT
```

Save changes:

```bash
sudo sh -c "iptables-save > /etc/iptables/iptables.rules"
sudo sh -c "ip6tables-save > /etc/iptables/ip6tables.rules"
```

```bash
sudo systemctl enable iptables.service
sudo systemctl enable ip6tables.service
```

Testing Rules:

```bash
sudo iptables -L -v -n
sudo ip6tables -L -v -n
```

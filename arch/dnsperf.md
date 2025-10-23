## dnsperf

```bash
sudo pacman -S dnsperf
```

Usage: `dnsperf [-f family] [-m mode] [-s server_addr] [-p port]`

Create a `queries.txt`:

```txt
google.com A
google.com AAAA
example.com A
example.com AAAA
cloudflare.com A
cloudflare.com AAAA
mozilla.org A
mozilla.org AAAA
github.com A
github.com AAAA
archlinux.org A
archlinux.org AAAA
dnscrypt.info A
dnscrypt.info AAAA
wikipedia.org MX
dns-oarc.net PTR
iana.org SRV
reddit.com CNAME
apple.com A
netflix.com MX
microsoft.com TXT
openai.com SOA
root-servers.net A
```

Run dnsperf targeting 127.0.0.1 for IPv4 DNS queries:

```bash
dnsperf -d queries.txt -s 127.0.0.1
```

The default dnsperf is single-threaded. Boost QPS with multiple clients:

```bash
dnsperf -d queries.txt -s 127.0.0.1 -c 10
```

Or if you want to test IPv6 explicitly, target ::1:

```bash
dnsperf -d queries.txt -s ::1
```

Since your dnsmasq forwards to 127.0.0.1#5353 and ::1#5353, make sure to adjust
the dnsperf port accordingly:

```bash
dnsperf -d queries.txt -s 127.0.0.1 -p 53
```

(dnsmasq listens on 53, dnscrypt-proxy on 5353)

To test dnscrypt-proxy directly on port 5353:

```bash
dnsperf -d queries.txt -s 127.0.0.1 -p 5353
```

## Testing External Resolvers

```bash
dnsperf -d queries.txt -s 9.9.9.9  # Quad9 DNS
dnsperf -d queries.txt -s 1.1.1.1  # Cloudflare DNS
dnsperf -d queries.txt -s 8.8.8.8  # Google DNS
```

- The fastest of the above resolvers was Cloudflare

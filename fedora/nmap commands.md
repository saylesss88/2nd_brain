# Nmap Command Examples

1. *Basic Nmap Scan against IP or host*
```bash
nmap 1.1.1.1
```
- Now if you want to scan a hostname, replace the IP for the host:
```bash
nmap recordedfuture.xyz
```
- This kind of scans, such as the Nmap scan host are perfect first steps.

2. *Nmap Ping Scan*
```bash
nmap -sp 192.168.5.0/24
```
- The most famous type of scan is the Nmap ping scan (so-called because it's often used to perform Nmap ping sweeps), and it's the easiest way to detect hosts on any network.

- The drawback of this ICMP-only type of scan is that remote hosts often block IP-based ping packets, so if you're unable to get solid results, we recommend switching to ARP-based requests for your scan.

3. *Scan specific ports or scan entire port ranges on a local or remote server*

```bash
nmap -p 80,443 8.8.8.8
```
4. *Scan multiple IP addresses*
```bash
nmap 1.1.1.1 8.8.8.8
```
You can also scan consecutive IP addresses:
```bash
nmap 1.1.1.1,2,3,4
```
This will scan 1.1.1.1, 1.1.1.2, 1.1.1.3, and 1.1.1.4



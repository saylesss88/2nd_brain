---
id: PORTS
aliases:
  - PORTS
tags:
  - linux
  - ports
---

# PORTS

Open ports serve critical applications but also pose security risks. Regularly
checking them is essential for server security and performance.

- The 6 primary methods for checking open ports are `netstat`, `ss`, `lsof`,
  `nmap`, `netcat`, and `PowerShell`.

- Understanding key ports like SSH (22), HTTP (80), and HTTPS (443) is important
  for identifying active services and ensuring they're correctly configured and
  secure.

- Knowing whether ports use `TCP` or `UDP` and configuring firewall rules can
  significantly enhance security and network functionality.

## Understanding Open Ports in Linux

Ports are like open doors that allow traffic to flow in and out of your server.
Each port has a unique number and serves as a designated point for specific
services or applications, such as web servers, email, and file transfers.

- Some ports are open by default to facilitate essential functions, while others
  may be closed for security purposes.

### TCP/UDP protocols, firewalls, and services

Transmission Control Protocol (TCP) and User Datagram Protocol (UDP) are the
main communication protocols that dictate how data is transmitted across the
network. `TCP` is a connection-oriented protocol, meaning it establishes a
reliable connection between the client and server before transmitting data.
Making it ideal for apps where data integrity is critical, such as web browsing
(port 80 for HTTP and port 443 for HTTPS) and file transfers.

`UDP` is a connectionless protocol, prioritizing speed over reliability, making
it suitable for streaming and real-time apps like DNS queries and VoIP.

**Firewalls**

A firewall acts as a security gatekeeper, filtering incoming and outgoing
traffic based on predefined rules. `iptables` and `firewalld` are two popular
tools for managing firewall rules, which can open, close, or restrict access to
specific ports. Firewall settings will determine which ports are accessible
externally. Proper firewall management is crucial to ensure only necessary ports
are open, reducing the potential for unauthorized access.

#### Common Linux Ports and their Services

- `Port 22 (SSH)`: Secure Shell, which enables secure remote access to the
  server.

- `Port 80 (HTTP)`: World Wide Web, which allows users to access web

- `Port 443 (HTTPS)`: Secure Hypertext Transfer Protocol, used by SSL/TLS to
  encrypt web traffic, protecting data exchanged between websites and users.

- `Port 21 (FTP)`: File Transfer Protocol, used for file transfers between
  computers on a network.

- `Port 25 (SMTP)`: Simple Mail Transfer Protocol, used for email delivery.

- `Port 3306 (MySQL)`: MySQL, a popular open-source relational database.

##### Nmap, lsof, netstat

Nmap (Network Mapper) is a popular open-source tool for scanning your network
for open ports and active services. It's often the go-to choice for network
administrators due to its detailed output and versatility.

```bash
nmap localhost
```

- This command will scan all commonly used ports on your server and return
  information on any that are open.

You can also specify a particular range of ports if you're looking for something
more specific:

```bash
nmap -p 1-65535 localhost
```

Using the `lsof` command:

```bash
sudo lsof -i -P -n
```

- This command lists all network connections without resolving hostnames (`-n`)
  and displays port numbers rather that service names (`-P`). Look for entries
  marked with "LISTEN" to see which ports are open for incoming traffic.

Using the `netstat` command:

```bash
sudo netstat -tuln
```

- This command displays all TCP (`-t`) and UDP (`-u`) ports in a listening state
  (`-l`), without resolving hostnames (`-n`), making it easy to see which ports
  are open.

```bash
sudo ss -tuln
```

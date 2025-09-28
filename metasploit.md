## Metasploit

1. Launch:

```bash
sudo mfconsole
```

2. `search`: The search command helps us to find specific modules, such as
   exploits or auxiliaries (helper modules). For modules related to scanning:

```bash
msf6 > search scanner
```

3. `info`: Learn more about a module, including its options and how it works.

```bash
msf6 > info auxiliary/scanner/portscan/tcp
```

4. `use`

```bash
msf6 > use auxiliary/scanner/portscan/tcp
msf6 auxiliary(scanner/portscan/tcp) >
```

5. `options`: Once a module is loaded with `use`, you can see a list of options
   using the `options` command:

```bash
msf6 auxiliary(scanner/portscan/tcp) > options
```

For example, the RHOSTS parameter is used to set the target IP address for
scanning. `scanme.nmap.org` lets us run port scans on that server, so let's use
that to run a scan.

We'll grab the IP address of the server from this ping command:

```bash
msf6 auxiliary(scanner/portscan/tcp) > ping scanme.nmap.org
[*] exec: ping scanme.nmap.org

PING scanme.nmap.org (2600:3c01::f03c:91ff:fe18:bb2f) 56 data bytes
64 bytes from scanme.nmap.org (2600:3c01::f03c:91ff:fe18:bb2f): icmp_seq=1 ttl=51 time=193 ms
64 bytes from scanme.nmap.org (2600:3c01::f03c:91ff:fe18:bb2f): icmp_seq=2 ttl=51 time=115 ms
64 bytes from scanme.nmap.org (2600:3c01::f03c:91ff:fe18:bb2f): icmp_seq=3 ttl=51 time=138 ms
64 bytes from scanme.nmap.org (2600:3c01::f03c:91ff:fe18:bb2f): icmp_seq=4 ttl=51 time=160 ms
64 bytes from scanme.nmap.org (2600:3c01::f03c:91ff:fe18:bb2f): icmp_seq=5 ttl=51 time=186 ms
```

We can see that the IP is `2600:3c01::f03c:91ff:fe18:bb2f`

```bash
msf6 auxiliary(scanner/portscan/tcp) > set RHOSTS 2600:3c01::f03c:91ff:fe18:bb2f
RHOSTS =>2600:3c01::f03c:91ff:fe18:bb2f
```

6. `run`: To run a module, we use the `run` command. Now that we've set the
   target IP, let's run the module to see if any ports are open.

```bash
msf6 auxiliary(scanner/portscan/tcp) > run

[+] [2600:3c01::f03c:91ff:fe18:bb2f] - 2600:3c01::f03c:91ff:fe18:bb2f:22 - TCP OPEN
[+] [2600:3c01::f03c:91ff:fe18:bb2f] - 2600:3c01::f03c:91ff:fe18:bb2f:80 - TCP OPEN
[*] [2600:3c01::f03c:91ff:fe18:bb2f] - Scanned 1 of 1 hosts (100% complete)
[*] Auxiliary module execution completed
```

We see that ports `22` and `80` are open. Tools like Nmap are better for
in-depth port scanning, but Metasploit offers modules for almost every segment
of a cybersecurity audit.

Check your own IP address:

```bash
curl ip.me
2601:881:8100:cff0:f1d:cf6f:3203:5ec1
```

```bash
msf6 auxiliary(scanner/portscan/tcp) > set RHOSTS 2601:881:8100:cff0:f1d:cf6f:3203:5ec1
```

```bash
run
[+] [2601:881:8100:cff0:f1d:cf6f:3203:5ec1] - 2601:881:8100:cff0:f1d:cf6f:3203:5ec1:2222 - TCP OPEN
[*] [2601:881:8100:cff0:f1d:cf6f:3203:5ec1] - Scanned 1 of 1 hosts (100% complete)
[*] Auxiliary module execution completed
```

7. `exit`: When you're done, type `exit` to leave the console.

### Nmap

```bash
~/notes󰏫 nmap -v -sn -6 2601:881:8100:cff0:f1d:cf6f:3203:5ec1
Starting Nmap 7.98 ( https://nmap.org ) at 2025-09-05 09:33 -0400
Initiating Ping Scan at 09:33
Scanning 2601:881:8100:cff0:f1d:cf6f:3203:5ec1 [2 ports]
Completed Ping Scan at 09:33, 0.00s elapsed (1 total hosts)
Initiating Parallel DNS resolution of 1 host. at 09:33
Completed Parallel DNS resolution of 1 host. at 09:33, 1.50s elapsed
Nmap scan report for 2601:881:8100:cff0:f1d:cf6f:3203:5ec1
Host is up (0.000054s latency).
Nmap done: 1 IP address (1 host up) scanned in 1.50 seconds
~/notes󰏫 nmap -v -r -6 2601:881:8100:cff0:f1d:cf6f:3203:5ec1
Starting Nmap 7.98 ( https://nmap.org ) at 2025-09-05 09:35 -0400
Initiating Ping Scan at 09:35
Scanning 2601:881:8100:cff0:f1d:cf6f:3203:5ec1 [2 ports]
Completed Ping Scan at 09:35, 0.00s elapsed (1 total hosts)
Initiating Parallel DNS resolution of 1 host. at 09:35
Completed Parallel DNS resolution of 1 host. at 09:35, 1.00s elapsed
Initiating Connect Scan at 09:35
Scanning 2601:881:8100:cff0:f1d:cf6f:3203:5ec1 [1000 ports]
Discovered open port 2222/tcp on 2601:881:8100:cff0:f1d:cf6f:3203:5ec1
Completed Connect Scan at 09:35, 1.20s elapsed (1000 total ports)
Nmap scan report for 2601:881:8100:cff0:f1d:cf6f:3203:5ec1
Host is up (0.00021s latency).
Not shown: 998 closed tcp ports (conn-refused)
PORT     STATE    SERVICE
53/tcp   filtered domain
2222/tcp open     EtherNetIP-1

Read data files from: /nix/store/8ad78i36mr9vbcrlxcihka556a3bixql-nmap-7.98/bin/../share/nmap
Nmap done: 1 IP address (1 host up) scanned in 2.23 seconds
```

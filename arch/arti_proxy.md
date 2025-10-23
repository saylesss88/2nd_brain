## Set up a SOCKS5 Proxy with Arti

> ❗️ NOTE: This is one example of using a SOCKS5 proxy to circumvent censorship
> or add additional privacy without the Tor Browser. You can also route other
> apps through the proxy, such as email clients, messaging apps, torrent
> clients, and more.

1. Clone the arti repo:

```bash
# clone the repo
git clone https://gitlab.torproject.org/tpo/core/arti.git

# navigate to the directory
cd arti
```

---

2. To build the Arti binary, compile the code and generate the executable run:

These are the safer build options so you can leave the arti repo in your home
directory without it leaking your username:

```bash
RUSTFLAGS="--remap-path-prefix $HOME/.cargo=.cargo --remap-path-prefix $(pwd)=." \
   cargo build --release -p arti
```

---

3. To allow Arti SOCKS proxy traffic you need to add a rule permitting incoming
   connections to port 9150.

For nftables, you would open `/etc/nftables.conf` and add:

```conf
chain input {
  # ...snip...

  # Allow Arti SOCKS proxy (port 9150)
    tcp dport 9150 ct state new accept

  # ...snip...
}
```

Enable it with `sudo nft -f /etc/nftables.conf`

---

4. To run Arti as a SOCKS proxy on port `9150`, execute:

```bash
./target/release/arti proxy
```

---

5. Configure LibreWolf/Firefox to use the Arti proxy:

Open LibreWolf or Firefox

Go to the menu and open `Preferences/Settings`.

Scroll to the bottom `Network Settings` section.

Click on "`Settings...`" under Network Settings.

In the connection settings dialog:

Select "`Manual proxy configuration`".

For "SOCKS Host", enter `127.0.0.1`.

For the port next to SOCKS Host, enter `9150`.

Select the SOCKS version 5 option (`SOCKS v5`).

Optionally check the box "`Proxy DNS when using SOCKS v5`" to route DNS queries
through the proxy for enhanced privacy.

Click "`OK`" to apply the settings.

---

6. Verify Your Proxy Setup Open a new tab and visit `https://dnsleaktest.com`
   and run an `Extended Test`.

Your IP address should now appear as a Tor exit node IP, indicating your traffic
is routed through the Arti proxy.

- Make sure Arti is running in its terminal or background before you start
  browsing.

- If you close the terminal or stop Arti, your browser will lose the proxy
  connection.

This setup only proxies the configured browser traffic; other apps are not
affected unless configured similarly.

This setup turns LibreWolf or Firefox into a Tor-enabled browser without
installing the Tor Browser Bundle, using the Arti SOCKS proxy instead. It can be
useful if you want to use a more customizable or alternative browser while still
accessing the Tor network securely.

> ⚠️ While using LibreWolf with the Arti SOCKS5 proxy provides network-level
> anonymity by routing traffic through the Tor network, it does not include the
> extensive browser-level privacy and security enhancements found in the
> official Tor Browser. For casual or moderate privacy needs the SOCKS proxy can
> be useful but for stronger anonymity guarantees and protection, the Tor
> Browser is recommended.

## Setup an Arti service to run in the background

Create a service file at `/etc/systemd/system/arti.service`:

Replace `your-username` with your username

```.service
[Unit]
Description=Arti Tor Proxy Service
After=network.target

[Service]
ExecStart=/home/your-username/arti/target/release/arti proxy
Restart=on-failure
User=jr
Group=jr
WorkingDirectory=/home/your-username/arti
Environment=RUSTFLAGS="--remap-path-prefix $HOME/.cargo=.cargo --remap-path-prefix $(pwd)=."

[Install]
WantedBy=multi-user.target
```

Enable & Start the service:

```bash
sudo systemctl enable arti
sudo systemctl start arti --now
```

Ensure its running:

```bash
sudo systemctl status arti
```

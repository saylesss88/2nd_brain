## Firewall

```bash
sudo pacman -S ufw
```

Default Policies

```bash
sudo ufw default deny incoming
sudo ufw default allow outgoing
```

Allow SSH for remote access if necessary:

```bash
sudo ufw allow ssh
# Or, by port #
sudo ufw allow 22/tcp
```

Allow Web Servers (HTTP and HTTPS):

```bash
sudo ufw allow http
sudo ufw allow https
# Or, by port numbers:
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
```

**Enable and Verify the Firewall**

```bash
sudo systemctl enable ufw --now
sudo ufw enable
```

Check the status and see the rules added:

```bash
sudo ufw status verbose
```

Delete a Rule:

```bash
sudo ufw delete allow 22/tcp
# Disable Firewall
sudo ufw disable
# Reset Rules
sudo ufw reset
```

## NFTables

The following is for Whonix KVM

Edit `/etc/nftables.conf`:

```conf
#!/usr/bin/nft -f
delete table inet filter
table inet filter {
  chain forward {
    type filter hook forward priority filter; policy drop;
    iifname "virbr1" accept comment "Whonix external bridge"
    iifname "virbr2" accept comment "Whonix internal bridge"
    oifname "virbr1" ct state established,related accept
  }
}
```

Save your rules and load with:

```bash
sudo nft -f /etc/nftables.conf
```

```bash
sudo nft list ruleset
```

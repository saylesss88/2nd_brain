# Hardening and Security Practices in NixOS

Securing your NixOS system begins with a philosophy of minimalism, explicit
configuration, and proactive control.

Start Minimal, Add Only What You Need Choose the minimal ISO or netboot
installer: Begin with NixOS’s minimal installation image. This gives you a base
system with only essential tools and no extras that could introduce
vulnerabilities.

Explicitly enable each service: In your `configuration.nix`, only enable
networking, SSH, desktop environments, and applications as needed. Remove or
avoid legacy daemons and sample services.

Principle of Least Privilege Limit installed software: Each program or service
added is potential attack surface. Install packages individually rather than
enabling broad module imports or convenience meta-packages.

Run services as unprivileged users: Wherever possible, configure system services
to run with a dedicated user and group, not as root.

Use NixOS’s fine-grained service options: For example, set systemd sandboxing
options (ProtectHome, PrivateTmp, NoNewPrivileges), and use NixOS modules’
user/group settings for daemons.

Secure the Boot & Init Process Enable Secure Boot: Use modules like lanzaboote
to enforce EFI Secure Boot, ensuring only signed kernels are loaded.

Encrypt your root and data partitions: Use LUKS+ext4 or btrfs for storage with
strong passphrases or keyfiles.

Keep the Attack Surface Small Disable unused features and daemons: Comment out
or set enable = false; for modules like CUPS, Samba, avahi, etc., if you don’t
need printing, filesharing, or zeroconf networking.

## Firewalls

NixOS includes an integrated firewall based on iptables/nftables.

Review listening ports: After each rebuild, use ss -tlpn or netstat to see which
services are accepting connections. Close or firewall anything unnecessary.

Use the built-in firewall: Enable and configure networking.firewall to allow
only explicitly required ports.

## Securing SSH

First of all, if you don't use SSH don't enable it in the first place. If you do
use SSH, it's important to understand what that opens you up to.

- [Mozilla OpenSSH guidelines](https://infosec.mozilla.org/guidelines/openssh.html)

Harden Authentication and Access Use key-based SSH, disable passwords: Set
services.openssh.settings.PasswordAuthentication = false; and use SSH keys
generated with strong algorithms (e.g., Ed25519).

Restrict SSH access: Use AllowUsers, limit sources with firewall rules, and
disable root login.

Use multi-factor authentication (MFA): For both SSH and desktop login,
optionally integrate U2F, TOTP, or smartcard support.

Manage secrets declaratively: Use tools like sops-nix to keep secrets encrypted
and versioned, never in the clear in your git repos.

System Update and Maintenance Regularly update and upgrade: Use nixos-rebuild
frequently and review channel or flake updates to patch vulnerabilities quickly.

Audit installed packages: Use nix-store --gc and nix-collect-garbage to minimize
leftover, unused software.

Automate security updates: Consider scheduled builds or notifications for NixOS
security advisories.

Monitoring, Logging, and Auditing Enable audit logging: Set
security.auditd.enable = true; for system-level event logging.

**Monitor denied accesses**: Configure `security.apparmor` or `security.selinux`
as a mandatory access control layer, and regularly check logs for AppArmor or
SELinux policy denials.

**Review logs with `journalctl`**: Check system logs for unauthorized access
attempts or configuration errors.

Advanced Hardening Implement sandboxing: For server workloads or exposed
applications, consider running them in systemd-nspawn, Firejail, or with user
namespaces for isolation.

Deploy mandatory access control (MAC): Enable and tune AppArmor or SELinux for
application-level confinement. Write or port profiles for critical apps and
services.

Control USB/Removable access: Use `services.usbguard` to restrict which USB
devices are accepted. Be particularly careful if your authentication keyfiles
are on USB devices.

## Encrypted DNS

```nix
# dnscrypt-proxy.nix
{
  config,
  lib,
  pkgs,
  inputs,
  ...
}: let
  blocklist_base = builtins.readFile inputs.oisd;
  extraBlocklist = '''';
  blocklist_txt = pkgs.writeText "blocklist.txt" ''
    ${extraBlocklist}
    ${blocklist_base}
  '';
  hasIPv6Internet = true;
  StateDirectory = "dnscrypt-proxy";
in {
  # Without systemd-resolved
  networking = {
    # Set DNS nameservers to the local host addresses for iPv4 (`127.0.0.1`) & iPv6 (::1)
    nameservers = ["127.0.0.1" "::1"];
    dhcpcd.extraConfig = "nohook resolv.conf";
    networkmanager.dns = "none";
    # networkmanager.dns = lib.mkIf config.networking.networkmanager.enable "none";
    # dhcpcd.extraConfig = lib.mkIf config.networking.dhcpcd.enable "nohook resolv.conf";
  };
  services.resolved.enable = lib.mkForce false;

  services.tor = {
    enable = true;
    client.enable = true;
    torsocks = {
      enable = true;
      allowInbound = false;
    };
    settings.SafeSocks = true;
    settings.TestSocks = true;
  };

  programs.firefox.policies.Preferences = {
    "network.proxy.socks" = builtins.head (builtins.split ":" config.services.tor.torsocks.server);
    "network.proxy.socks_port" = lib.last (builtins.split ":" config.services.tor.torsocks.server);
    "network.proxy.socks_version" = 5;
    "network.connectivity-service.DNSv4.domain" = "127.0.0.1";
    "network.connectivity-service.DNSv6.domain" = "::1";
    "network.dns.forceResolve" = true;
    "network.dns.localDomains" = "::1";
    "network.dns.preferIPv6" = true;
    "media.peerconnection.enabled" = false;
  };

  systemd.services.dnscrypt-proxy2.serviceConfig = {
    after = [
      "network-online.target"
      "tor.service"
    ];
    wants = [
      "network-online.target"
      "tor.service"
    ];
  };

  # Static UID/GID (non-dynamic) best for production
  # systemd.services.dnscrypt-proxy2.serviceConfig = {
  #   User = "dnscrypt-proxy";
  #   Group = "dnscrypt-proxy";
  #   DynamicUser = lib.mkForce false;
  # };

  # users.users.dnscrypt-proxy = {
  #   isSystemUser = true;
  #   group = "dnscrypt-proxy";
  #   uid = 199;
  # };

  # users.groups.dnscrypt-proxy = {
  #   gid = 199;
  # };

  services.dnscrypt-proxy2 = {
    enable = true;
    settings = {
      ipv6_servers = hasIPv6Internet;
      block_ipv6 = ! hasIPv6Internet;
      blocked_names.blocked_names_file = blocklist_txt;
      require_dnssec = true;
      dnscrypt_servers = true;
      doh_servers = true;
      odoh_servers = false;
      require_nolog = false;
      require_nofilter = true;
      force_tcp = true;

      sources.public-resolvers = {
        urls = [
          "https://raw.githubusercontent.com/DNSCrypt/dnscrypt-resolvers/master/v3/public-resolvers.md"
          "https://download.dnscrypt.info/resolvers-list/v3/public-resolvers.md"
        ];
        minisign_key = "RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3"; # See https://github.com/DNSCrypt/dnscrypt-resolvers/blob/master/v3/public-resolvers.md
        cache_file = "/var/lib/${StateDirectory}/public-resolvers.md";
      };

      # You can choose a specific set of servers from https://github.com/DNSCrypt/dnscrypt-resolvers/blob/master/v3/public-resolvers.md
      #server_names = [ ... ];
    };
  };
  systemd.services.dnscrypt-proxy2.serviceConfig = {
    StateDirectory = StateDirectory;
  };
}
```

```bash
sudo systemctl status dnscrypt-proxy2
# verify that dnscrypt-proxy is listening
sudo ss -lnp | grep 53
# Test a DNS query, if you get valid responses it's working
dig @127.0.0.1 example.com +short
# check the logs
sudo journalctl -u dnscrypt-proxy2
```

Firefox and Tor Firefox is configured to use Tor via SOCKS5 proxy for all
traffic (including DNS lookups).

All Firefox requests (HTTP, HTTPS, DNS) go through your local Tor SOCKS5 proxy
(typically 127.0.0.1:9050).

All Firefox DNS queries are forced to pass through the proxy (and thus Tor), so
your browser’s DNS is very private.

WebRTC is disabled to prevent local IP leaks.

dnscrypt-proxy2 dnscrypt-proxy2 acts as your local DNS caching resolver.

All DNS clients on your system (dig, curl, most apps, except Firefox which has
its own proxy) use dnscrypt-proxy2.

dnscrypt-proxy2 filters ads/trackers (using oisd), enforces DNSSEC, and uses
encrypted transports (DNS-over-HTTPS/DoH, DNSCrypt, optionally
DNS-over-TLS/DoT).

dnscrypt-proxy2 does not route its upstream queries through Tor by default, DNS
lookups from dnscrypt-proxy2 to the real Internet go directly to the resolver
(e.g., Cloudflare, Quad9), not via Tor. This means your ISP and the resolver can
see your IP and your DNS queries.

There are no proxy = ... or socks_proxy = ... settings in your dnscrypt-proxy2
config, so it will not use Tor.

Tor Tor is enabled, which means you have a local SOCKS5 proxy (127.0.0.1:9050,
127.0.0.1:9051, etc.).

Tor is used by Firefox (via browser proxy settings), but not by dnscrypt-proxy2.

## USB interfaces

## Resources

- [AppArmor and apparmor.d on NixOS](https://hedgedoc.grimmauld.de/s/hWcvJEniW#)

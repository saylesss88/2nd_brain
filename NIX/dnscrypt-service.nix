{ config, lib, pkgs, ... }:

let
  # Put the generated file somewhere mutable (NOT /nix/store).
  stateDir = "/var/lib/dnscrypt-proxy";
  blocklist = "${stateDir}/blocklist.txt";

  # Example allowlist (optional); you can also point at a real file in /etc.
  allowlist = pkgs.writeText "domains-allowlist.txt" ''
    # put domains here, one per line
    example.com
  '';
in
{
  # 1) Tell dnscrypt-proxy to use the generated blocklist.
  # NixOS converts `services.dnscrypt-proxy.settings` into the TOML config. [page:1]
  services.dnscrypt-proxy = {
    enable = true;
    settings = {
      blocked_names = {
        blocked_names_file = blocklist;
        log_file = "/var/log/dnscrypt-proxy/blocked-names.log";
      };
    };
  };

  # Optional but nice: give dnscrypt-proxy a StateDirectory (/var/lib/dnscrypt-proxy).
  # (The NixOS wiki example sets this via systemd serviceConfig.) [page:1]
  systemd.services.dnscrypt-proxy.serviceConfig.StateDirectory = "dnscrypt-proxy";

  # 2) Oneshot generator service.
  systemd.services.dnscrypt-filterlist-update = {
    description = "DNSCrypt Filterlist Update";
    serviceConfig = {
      Type = "oneshot";
      User = "root";
      # WorkingDirectory is a normal systemd.exec setting if you need it. [web:180]
      # WorkingDirectory = "...";
    };

    # Provide tools in PATH for the script.
    path = with pkgs; [ dnscrypt-proxy coreutils systemd ];

    script = ''
      set -euo pipefail
      install -d -m 0755 ${lib.escapeShellArg stateDir}

      # Equivalent to your: generate-domains-blocklist -a ... -o ...
      generate-domains-blocklist \
        -a ${lib.escapeShellArg allowlist} \
        -o ${lib.escapeShellArg blocklist}

      sleep 2
      systemctl restart dnscrypt-proxy.service
    '';
  };

  # 3) Timer: run 15 min after boot, then every 5h after last run. [page:0][web:179]
  systemd.timers.dnscrypt-filterlist-update = {
    wantedBy = [ "timers.target" ];
    timerConfig = {
      OnBootSec = "15min";
      OnUnitActiveSec = "5h";
      Unit = "dnscrypt-filterlist-update.service";
    };
  };
}


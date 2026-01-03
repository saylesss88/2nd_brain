# /etc via overlay filesystem

What it changes With system.etc.overlay.enable = true; and
system.etc.overlay.mutable = false;, /etc is presented as a read‑only overlay
backed by Nix‑managed configuration, and the upperdir used for runtime changes
lives under /.rw-etc/upper. On each nixos-rebuild switch, the lower (immutable)
layer is atomically replaced, while any ad‑hoc edits under /etc are no longer
visible unless you go look in that upperdir.​

Security implications Integrity / drift reduction: making /etc effectively
read‑only in normal operation makes it harder for software or an attacker
running with limited privileges to silently persist config changes there; they
have to go through Nix or tamper with the overlay plumbing, which is more
visible and auditable.​

Reproducibility: it pushes you further toward “all config in Nix, not in /etc”,
which improves reproducibility and makes config diffing and rollback more
trustworthy after an incident.​

Limitations: this does not stop anything that already has root from modifying
the overlay mount or the underlying dirs, and some services still expect to
write into /etc, so you may hit breakage like the reported sing-box failure when
/etc is read‑only.​

So turning it on, especially with mutable = false, nudges the system closer to
an immutable‑config model and reduces accidental or low‑effort tampering, but it
should be combined with other controls (MAC, sandboxing, restricted sudo) rather
than treated as the main security boundary.​

- [etc via overlay](https://nixos.org/manual/nixos/stable/#sec-etc-overlay)

# Clippy Commands

Standard strict check:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Even stricter with pedantic lints:

```bash
cargo clippy --all-targets --all-features -- -W clippy::pedantic -D warnings
```

Recommended for serious projects:

```bash
cargo clippy --all-targets --all-features -- \
  -D warnings \
  -W clippy::pedantic \
  -W clippy::nursery \
  -W clippy::cargo
```

- `--all-targets` - Check lib, bins, tests, examples, benches

- `--all-features` - Check with all feature flags enabled

- `-D warnings` - Deny all warnings (fail the build)

- `-W clippy::pedantic` - Enable extra style lints

- `-W clippy::nursery` - Enable experimental lints (may have false positives)

- `-W clippy::cargo` - Check `Cargo.toml` for best practices

Auto-fix issues:

```bash
cargo clippy --fix --all-targets --all-features
```

For NixOS projects, add this to your `flake.nix`:

```nix
devShells.default = pkgs.mkShell {
  packages = [ pkgs.clippy ];
  shellHook = ''
    alias clip='cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic'
  '';
};
```

Then just run `clip` in your dev shell.

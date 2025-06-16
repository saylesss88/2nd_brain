## Overriding a rust source

```nix
self: super: {
    airshipper = super.airshipper.overrideAttrs (old: rec {
        version = "0.9.0";
        src = self.fetchFromGitLab {
            owner = "Veloren";
            repo = "airshipper";
            rev = "v${version}";
            hash = "";
        };
    });
}
```

When you look at the airshipper `package.nix` you'll see the `cargoHash` which
is the issue.

```nix
# airshipper/package.nix
rustPlatform.buildRustPackage {
  pname = "airshipper";
  inherit version;

  src = fetchFromGitLab {
    owner = "Veloren";
    repo = "airshipper";
    rev = "v${version}";
    hash = "sha256-MHwyXCAqdBzdJlYzSeUXr6bJdTVHcjJ/kGcuAsZCCW8=";
  };
  useFetchCargoVendor = true;
  cargoHash = "sha256-TkeB939zV5VvqICFqJd/7uX+ydXyEQOJ3sYQbHbZhP0="; # here
```

We want to know more about `buildRustPackage`, to find it in Nixpkgs:

```bash
rg 'buildRustPackage ='

pkgs/development/compilers/rust/make-rust-platform.nix
33:      buildRustPackage = callPackage ../../../build-support/rust/build-rust-package {

pkgs/by-name/te/tectonic-unwrapped/package.nix
24:  buildRustPackage = rustPlatform.buildRustPackage.override {
```

`cargoDeps` is what deals with the hash

```nix
# build-rust-package/default.nix
# ... snip ...
    // {
      cargoDeps =
        if cargoVendorDir != null then
          null
        else if cargoDeps != null then
          cargoDeps
        else if cargoLock != null then
          importCargoLock cargoLock
        else if args.cargoHash or null == null then
          throw "cargoHash, cargoVendorDir, cargoDeps, or cargoLock must be set"
        else
          fetchCargoVendor (
            {
              inherit
                src
                srcs
                sourceRoot
                cargoRoot
                preUnpack
                unpackPhase
                postUnpack
                ;
              name = cargoDepsName;
              patches = cargoPatches;
              hash = args.cargoHash;
            }
            // depsExtraArgs
          );
          inherit buildAndTestSubdir;
# ... snip ...
```

This does `fetchCargoVendor` and injects the hash with `hash = args.cargoHash`.
We will need to override this function

```nix
self: super: {
    airshipper = super.airshipper.overrideAttrs (old: rec {
        version = "0.9.0";
        # .overrideAttrs to override cargoDeps
        # .overrideAttrs to override src and outputHash
        cargoDeps = old.cargoDeps.overrideAttrs (old: {
            inherit src version;
            outputHash = "";
        });
        src = self.fetchFromGitLab {
            owner = "Veloren";
            repo = "airshipper";
            rev = "v${version}";
            hash = "";
        };
    });
}
```

```bash
nix repl
nix-repl> :a import ./. { overlays = [ (import ./airshipper.nix) ]; }
nix-repl> airshipper
«derivation /nix/store/5lqvazyid8g3524p6r4hdrlj91jqazv5-airshipper-0.14.0.drv»
nix-repl> airshipper.cargoDeps
«derivation /nix/store/d0wlkmdyp304v61mlbyz6z3dgvsld6xm-airshipper-0.16.0-vendor.drv»
# the outputHash is its own derivation
nix-repl> airshipper.drvAttrs.cargoDeps.drvAttrs.outputHash
"sha256-GLXFGFGq0nejpr1E0LETU1rts2dVyfnggmdN2uKZ7mE="
nix-repl> airshipper.drvAttrs.cargoDeps.drvAttrs.version
"0.14.0"
nix-repl> airshipper.drvAttrs.cargoDeps.drvAttrs.outputs
[ "out" ]
nix-repl> airshipper.drvAttrs.cargoDeps.drvAttrs.nativeBuildInputs
[
  «derivation /nix/store/qgzcb17cn426vm8da7kb92fg9w73xyhh-fetch-cargo-vendor-util.drv»
  «derivation /nix/store/fc9qr6i3xyalwb4pnfclgkwvxasr12xm-cargo-1.86.0.drv»
  «derivation /nix/store/b6zn096s7i53gar2d59d7018frcpm61c-replace-workspace-values.drv»
]
```

`:a` adds just the scope we declare and avoids adding the entire scope of
nixpkgs

```bash
nix-repl> :l <nixpkgs>
nix-repl> airshipper // { type = ""; }
nix-repl> :p airshipper // { type = ""; }
nix-repl> :p airshipper.drvAttrs # useful
nix-repl> builtins.derivation { name = "foo"; builder = "/bin/sh"; system = builtins.currentSystem; }
«derivation /nix/store/9d794zbx10n43fa6zaqf33pfymdrm4r2-foo.drv»

## Derivations

nix-repl> :b builtins.derivation { name = "foo"; builder = "/bin/sh"; system = builtins.currentSystem; }
error: builder for '/nix/store/9d794zbx10n43fa6zaqf33pfymdrm4r2-foo.drv' failed to produce output path for output 'out' at '/nix/store/9d794zbx10n43fa6zaqf33pfymdrm4r2-foo.drv.chroot/root/nix/store/xpcvxsx5sw4rbq666blz6sxqlmsqphmr-foo'
nix-repl> :b builtins.derivation { name = "foo"; builder = "/bin/sh"; args = ["-c" "touch $out" ]; system = builtins.currentSystem; }error: builder for '/nix/store/zap20wdqmym4d20hwjn144d53caq4h3d-foo.drv' failed with exit code 127;
       last 1 log lines:
       > sh: touch: not found
       For full logs, run:
         nix log /nix/store/zap20wdqmym4d20hwjn144d53caq4h3d-foo.drv
[0 built (2 failed), 0.0 MiB DL]
nix-repl> :b builtins.derivation { name = "foo"; builder = "/bin/sh"; args = ["-c" ">$out" ]; system = builtins.currentSystem; }
This derivation produced the following outputs:
  out -> /nix/store/nsf2kzah7sc8iv1g03f1wd7k37b8sjj1-foo
nix-repl> drv = builtins.derivation { name = "foo"; builder = "/bin/sh"; args = ["-c" ">$out" ]; system = builtins.currentSystem; }

nix-repl> drv
«derivation /nix/store/4pg0xm6s4f9lsqmr50vaxg07nccjybkp-foo.drv»

nix-repl> :b drv

This derivation produced the following outputs:
  out -> /nix/store/nsf2kzah7sc8iv1g03f1wd7k37b8sjj1-foo
# this is what you override with overrideAttrs
nix-repl> drv.drvAttrs
{
  args = [ ... ];
  builder = "/bin/sh";
  name = "foo";
  system = "x86_64-linux";
}
nix-repl> drv = builtins.derivation { name = "foo"; builder = "/bin/sh"; outputs = ["out" "lib"]; args = ["-c" ">$out; >$lib" ]; syst
nix-repl> :b drv

This derivation produced the following outputs:
  lib -> /nix/store/nzf9di66b1hb5jdl046xzc34kw3s92d3-foo-lib
  out -> /nix/store/a3anrssrkjpdq8df05z68l0jpax13fqz-foo

nix-repl> drv.out
drv.out         drv.outPath     drv.outputName  drv.outputs
nix-repl> drv.out
drv.out         drv.outPath     drv.outputName  drv.outputs
nix-repl> drv.out
«derivation /nix/store/dfk369laws8haiv5q45z54zp19251mqm-foo.drv»

nix-repl> drv.out
drv.out         drv.outPath     drv.outputName  drv.outputs
nix-repl> drv.outputs
[
  "out"
  "lib"
]
nix-repl> drv.out.out.out.lib.lib
«derivation /nix/store/dfk369laws8haiv5q45z54zp19251mqm-foo.drv»
nix-repl> drv = builtins.derivationStrict { name = "foo"; builder = "/bin/sh"; outputs = ["out" "lib"]; args = ["-c" ">$out; >$lib" ]
nix-repl> :p drv
{
  drvPath = "/nix/store/dfk369laws8haiv5q45z54zp19251mqm-foo.drv";
  lib = "/nix/store/nzf9di66b1hb5jdl046xzc34kw3s92d3-foo-lib";
  out = "/nix/store/a3anrssrkjpdq8df05z68l0jpax13fqz-foo";
}
nix-repl> :b drv // { name = "foo"; type = "derivation"; }
nix-repl> drv
{
  drvPath = "/nix/store/dfk369laws8haiv5q45z54zp19251mqm-foo.drv";
  lib = "/nix/store/nzf9di66b1hb5jdl046xzc34kw3s92d3-foo-lib";
  out = "/nix/store/a3anrssrkjpdq8df05z68l0jpax13fqz-foo";
}

nix-repl> :q

  ~/notes  2m21s
❯󰏫 cat /nix/store/dfk369laws8haiv5q45z54zp19251mqm-foo.drv
───────┬─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
       │ File: /nix/store/dfk369laws8haiv5q45z54zp19251mqm-foo.drv
───────┼─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
   1   │ Derive([("lib","/nix/store/nzf9di66b1hb5jdl046xzc34kw3s92d3-foo-lib","",""),("out","/nix/store/a3anrssrkjpdq8df05z68l0jpax13
       │ fqz-foo","","")],[],[],"x86_64-linux","/bin/sh",["-c",">$out; >$lib"],[("builder","/bin/sh"),("lib","/nix/store/nzf9di66b1hb
       │ 5jdl046xzc34kw3s92d3-foo-lib"),("name","foo"),("out","/nix/store/a3anrssrkjpdq8df05z68l0jpax13fqz-foo"),("outputs","out lib"
       │ ),("system","x86_64-linux")])
───────┴─────────────────────────────────────────
```

To see the implimentation of the derivation primitive you would first need to
clone the [nix repo](https://github.com/NixOS/nix) and go to
`nix/src/libexpr/primops/derivation.nix`

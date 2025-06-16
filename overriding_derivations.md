```nix
# test.nix
let
  pkgs = import <nixpkgs> {
    overlays = [
      (final: prev: {
        hello = prev.hello.override {
          fetchurl = throw "not fetchurl";
        };
      })
    ];
  };
in
  pkgs.hello
```

```bash
nix-build test.nix
```

```nix
let
  pkgs = import <nixpkgs> {
    overlays = [
      (final: prev: {
        hello =
          (prev.hello.override {
            fetchurl = throw "not fetchurl";
          }).overrideAttrs (oldAttrs: {
            src = oldAttrs.src;

            nativeBuildInputs =
              (oldAttrs.nativeBuildInputs or [])
              ++ [
                final.jq
              ];
          });
      })
    ];
  };
in pkgs.hello
```

```nix
stdenv.mkDerivation (finalAttrs: {
  pname = "hello";
  version = "2.12.2";

  src = fetchurl {
    url = "mirror://gnu/hello/hello-${finalAttrs.version}.tar.gz";
    hash = "sha256-WpqZbcKSzCTc9BHO6H6S9qrluNE72caBm0x6nc4IGKs=";
  };
```

Here `mkDerivation` takes a function and returns an attribute. This is a newer
style of doing things allowing you to avoid using `rec` and avoid rewriting
duplicate values.

This is most likely not what you want. In order to properly change the version
of a package, override both the `version` and `src` attributes:

````nix
    hello.overrideAttrs (oldAttrs: rec {
      version = "1.0.0";
      src = pkgs.fetchurl {
        url = "mirror://gnu/hello/hello-${version}.tar.gz";
        hash = "...";
      };
    })
```

```bash
nix repl
nix-repl> hello = import ./test.nix
nix-repl> hello
nix-repl> hello.src.url
"mirror://gnu/hello/hello-2.12.tar.gz"

```
````

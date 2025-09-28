---
id: firstFlake
aliases: []
tags: []
---

# First Flake

To create a Hello World and distribute it as a flake:

```bash
git init hello
cd hello
echo 'int main() { printf("Hello World"); }' > hello.c
git add hello.c
```

To turn this Git repository into a flake, we add a `flake.nix` at the root of
the repository:

```nix
{
  description = "A flake for building Hello World";

  inputs.nixpkgs.url = github:NixOS/nixpkgs/nixos-20.03;

  outputs = { self, nixpkgs }: {

    defaultPackage.x86_64-linux =
      # Notice the reference to nixpkgs here.
      with import nixpkgs { system = "x86_64-linux"; };
      stdenv.mkDerivation {
        name = "hello";
        src = self;
        buildPhase = "gcc -o hello ./hello.c";
        installPhase = "mkdir -p $out/bin; install -t $out/bin hello";
      };

  };
}
```

The command `nix flake init` creates a basic `flake.nix` for you.

Any file not tracked by Git is invisible during Nix evaluation, so:

```bash
git add flake.nix
```

And see if it builds:

```bash
nix build
warning: creating lock file '/home/eelco/Dev/hello/flake.lock'
warning: Git tree '/home/eelco/Dev/hello' is dirty

$ ./result/bin/hello
Hello World
```

What does the stuff in the `flake.nix` mean?

-   The `description` attribute is a one-line description shown by `nix flake
metadata`.

-   The `inputs` attribute specifies other flakes that this flake depends on.
    These are fetched by Nix and passed as arguments to the `outputs` attribute.

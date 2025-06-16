---
id: usingMake
aliases:
  - usingMake
tags: []
---

# usingMake

The `Makefile` is the key to the build process. In its simplest form, a Makefile
is a script for compiling or building the "binaries", the executable portions of
a package. The Makefile can also provide a means of updating a software package
without having to recompile every single source file in it.

- At some point, the Makefile launches `cc` or `gcc`. This is actually a
  preprocessor, a C (or C++) compiler, and a linker, invoked in that order. This
  process converts the source into the binaries, the actual executables.

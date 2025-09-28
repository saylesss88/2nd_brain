---
id: The main package
aliases:
  - The main package
tags: []
---

# The main package

In order for a bunch of Go source code to produce an executable, there has to
be at least some code in a special package named `main`.

Since we can't have more than one package in the same folder, we'll need to put
the `main` package in a subfolder of your project.

Because the `main` package produces a _command_ (that is executable), it's
conventional to name this folder `cmd`.(short for "command")

It's not enough to just declare a `main` package, whatever your program does
it has to _start_ somewhere, and the Go compiler needs to know where that
should be.

The way to tell it is by defining a special function, also named `main`.

The flow control in a Go program begins at the beginning of the `main` funct-
ion, and works through it in statement order, perhaps calling other functions
along the way.

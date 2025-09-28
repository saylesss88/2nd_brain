---
id: lifetimes
aliases: [generics]
---

# Lifetimes

Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.

- Every reference in Rust has a _lifetime_, which is the scope for which that reference is valid.

  - Most of the time lifetimes are implicit and inferred, just like most of the time, types are inferred.
  - We must annotate lifetimes when the lifetimes of references could be related in a few ways.

## Preventing Dangling References with Lifetimes

The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it's intended to reference.



## Hardened malloc

`hardened_malloc` is a hardened replacement for glibc's malloc().

`/etc/ld.so.preload`:

```preload
libhardened_malloc.so
```

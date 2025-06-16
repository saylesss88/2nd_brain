[---](2024-09-21_---.md)--
id: files
aliases:

- Startup/Shutdown Files
  tags: []

---

# Startup/Shutdown Files

Commands are first read from `/etc/zshenv`; this cannot be overridden.
Subsequent behavior is modified by the `RCS` and `GLOBAL_RCS` options; the
former affects startup filed, while the second only affects global startup
files.(Those with a path starting with `/`)

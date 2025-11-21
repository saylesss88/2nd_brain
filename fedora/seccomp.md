# seccomp

seccomp (short for secure computing) is a computer security facility in the
Linux kernel. seccomp allows a process to make a one-way transition into a
"secure" state where it cannot make any system calls except
`exit()`,`sigreturn()`,`read()`, and `write()` to already-open file descriptors.

- [Wikipedia seccomp](https://en.wikipedia.org/wiki/Seccomp)

## Flatpak Basics

### Runtimes

Runtimes provide the basic dependencies used by applications. Each application
must be built against a runtime, and this runtime must be installed on the host
system for the application to run. (Flatpak can automatically install the
required runtime for an application.) Multiple runtimes and different versions
of the same runtime can be installed alongside each other.

Runtimes are distribution agnostic and do not depend on a particular
distribution version. This means that they provide a stable, cross-distribution
base for applications and allow applications to work irrespective of operating
system updates.

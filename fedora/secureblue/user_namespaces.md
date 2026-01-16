## User namespaces

[User namespace](<https://en.wikipedia.org/wiki/Linux_namespaces#User_ID_(user)>)
are a kernel feature introduced in kernel version 3.8. When an unprivileged user
asks the kernel to create a namespace, the kernel needs to permit that user to
do so. Whether this is permitted by the kernel is controlled via a sysctl flag.

There is a long history of vulnerabilities made possible by allowing this
functionality for unprivileged users ever since its introduction. Given this
history, you might think we should just disable this functionality altogether.
However, if this functionality is disabled globally, then flatpak can’t
function.

To mitigate this, we first revoke user_namespace privileges from the unconfined
domain in our SELinux policy. Then, we confine flatpak and grant it
user_namespace privileges. We do the same for Trivalent. This allows them to
create user namespaces while keeping them globally disabled by default. We don’t
do this for Bubblewrap or Podman directly because the syscall filters for both
are weak by default. If you need container domain userns (e.g., for Distrobox),
you can enable it with ujust toggle-container-domain-userns-creation. If you
need to use any other software that requires user namespace creation privileges
(e.g., Bubblejail), you can enable it with ujust
toggle-unconfined-domain-userns-creation. But keep in mind that this is a
security degradation.

- [RedHat user_namespaces & containers](https://access.redhat.com/articles/5946151)

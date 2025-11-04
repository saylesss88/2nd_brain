## SELinux

Check the status of SELinux on your system:

```bash
sestatus
SELinux status:                 enabled
SELinuxfs mount:                /sys/fs/selinux
SELinux root directory:         /etc/selinux
Loaded policy name:             targeted
Current mode:                   enforcing
Mode from config file:          enforcing
Policy MLS status:              enabled
Policy deny_unknown status:     allowed
Memory protection checking:     actual (secure)
Max kernel policy version:      35
```

## SELinux states and modes

SELinux can run in one of three modes: disabled, permissive, or enforcing.

In permissive mode, the system acts as if SELinux is enforcing the loaded
security policy, including labeling objects and emmitting access denial
entries in the logs, but it doesn't actually deny any operations. Useful
for SELinux policy development.

Switch Modes:



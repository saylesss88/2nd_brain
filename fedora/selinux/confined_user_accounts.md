# Confined User Accounts

**Confined user accounts** are Linux users mapped to restricted SELinux "user"
domains. While most users are assigned `unconfined_u` (minimal restrictions),
mapping to a confined user type (like `user_u`, `staff_u`, or custom domains)
enforces SELinux policies on all their activities.

This means processes run by these users are subject to executable and writable
memory checks, cannot access files or processes outside their allowed domains,
and cannot perform actions unless explicitly allowed by the SELinux policy.

1. Create a new user mapped to a SELinux User

```bash
run0 useradd -Z staff_u <username>
```

2. Set a password for the new user

```bash
run0 passwd <username>
```

Verify the assigned SELinux User Context

Switch to the user:

```bash
su - <username>
# Check the SELinux user context applied
id -Z
# Should output
staff_u:staff_r:staff_t:s0-s0:c0.c1023
```

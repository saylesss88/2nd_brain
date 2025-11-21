# Intro to Selinux

SELinux was originally a development project from the National Security Agency
and originally implemented as a loadable kernel module.

AppArmor, enabled by default on recent Ubuntu versions and SELinux, enabled by
default on Android 5.0+ and Red Hat/Fedora.

## SELinux architecture -- the basics

The SELinux architecture can be split into four main components.

1. Firstly, a **Subject** must request access to take an action. In most cases,
   the subject is a process that is requesting access to a resource. Access can
   be controlled via Access Vector Rules.

2. Second, an **Object Manager (OM)** that controls the access of the subject.
   It will query the Security Server in order to allow or deny actions.

3. **Security Server**--the security server makes decisions based on the
   Security Policy and returns an answer.

4. **Access Vector Cache (AVC)**--this is a cache that stores the decisions of
   the security server in order to speed up performance.

```bash
# Standard file permissions
ls -l example
# SELinux context
ls -Z /etc/passwd
.rw-r--r-- 397 root system_u:object_r:passwd_file_t:s0 27 Oct 08:25  /etc/passwd
```

The security context format looks as follows:

```bash
user:role:type:[:range]
```

- `user`: The user represents a SELinux user. A SELinux user is separate from a
  Linux user but can be assigned multiple Linux users and helps bridge the gap
  between the Linux world and SELinux world. The names for SELinux users will
  often end in `_u`.

```bash
seinfo -u
```

```bash
run0 semanage login -l
```

## SELinux LOGS

When a subject, (for example, an application), attempts to access an object (for
example, a file), the policy enforcement server in the kernel checks an access
vector cache (AVC), where subject and object permission are cached. If a
decision cannot be made based on data in the AVC, the request continues to the
security server, which looks up the security context of the application and the
file in a matrix.

Permission is granted or denied, with an `avc: denied` message detailed in
`/var/log/messages` if permission is denied.

List all denials

```bash
run0 ausearch -m AVC
```

## Multi-Category Security (MCS)

Multi-Category Security (MCS) is an enhancement to SELinux, and allows users to
label files with categories. These categories are used to further constrain
Discretionary Access Contron (DAC) and Type Enforcement (TE) logic.

MCS labeling consists of configuring a set of categories, which are simply text
labels, such as "Company_Confidential", and then assigning users to those
categories. The administrator first configures the categories, then assigns
users to them as required.

A system in a home environment may have one category of "Private", and be
configured so that only trusted local users are assigned to this category.

## Multi-Level Security (MLS)

The term multi-level arises from the defense community's security
classifications: Confidential, Secret, and Top Secret.

## Security policy

The SELinux Policy is the set of rules that guide the SELinux security engine.
It defines _types_ for file objects and _domains_ for processes. It uses roles
to limit the domains that can be entered, and has user identities to specify the
roles that can be attained. In essence, types and domains are equivalent, the
difference being that types apply to objects while domains apply to processes.

A type is a way of grouping items based on their similarity

SELinux policies aren't usually created from scratch, instead you can use a
baseline policy, called a reference policy, which provides classes and rules
that utilize the core Linux components. The reference policy originated from the
NSA, was maintained by Tresys and is now located in the SELinuxProject github
organization.

## Usage

If you use the `cp` command without any additional command-line arguments, a
copy of the file is created in the new location using the default type of the
creating process and the target directory.

Use the `-Z` option to specify the label for the new file:

```bash
cp -Z user_u:object_r:user_home_t foo /tmp
ls -Z /tmp/foo
```

Moving files with `mv` retains the original type associated with the file. This
may cause problems, confusion, or minor insecurity.

Check a Process ID. Most of the processes are running in the `unconfined_t`
domain:

```bash
ps auxZ
```

Checking a User ID

```bash
id -Z
unconfined_u:unconfined_r:unconfined_t:s0-s0:c0.c1023
```

Check a File ID

```bash
cd /etc
ls -Z h* -d
drwxr-xr-x   - root system_u:object_r:cupsd_etc_t:s0    10 Nov 18:34  hp
.rw-r--r--   9 root system_u:object_r:etc_t:s0          10 Nov 18:34 󱁻 host.conf
.rw-r--r--   6 root system_u:object_r:hostname_etc_t:s0 23 Oct 07:51 󱁻 hostname
.rw-r--r-- 384 root system_u:object_r:net_conf_t:s0 10 Nov 18:34 hosts
```

Use the `restorecon` command to restore files to the default values according to
the policy.

## Viewing the Status of SELinux

```bash
sestatus
# Include info about the security contexts
sestatus -v
```

```bash
sestatus -b | grep httpd | grep on$
httpd_builtin_scripting                     on
httpd_enable_cgi                            on
```

### Resources

- [Intro to SELinux](https://github.blog/developer-skills/programming-languages-and-frameworks/introduction-to-selinux/)

- [SELinux Overview Red Hat](https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/5/html/deployment_guide/ch-selinux?utm_source=opensourcewatch.beehiiv.com&utm_medium=referral&utm_campaign=everything-you-wanted-to-know-about-selinux-but-were-afraid-to-run)

- [sysadmin's guide to SELinux](https://opensource.com/article/18/7/sysadmin-guide-selinux)

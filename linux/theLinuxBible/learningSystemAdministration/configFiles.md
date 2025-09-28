---
title: "Config Files"
alias: "/linux/theLinuxBible/learningSystemAdministration/configFiles"
---

# Config Files

The two major locations of configuration files are your home directory (where
your personal config files are kept) and the `/etc` directory (which holds
system-wide config files).

Some useful configuration files:

- `/etc/cron*`: Directory in this set contain files that define how the `crond`
  utility runs applications on a daily (`cron.daily`), hourly (`cron.hourly`),
  monthly (`cron.monthly`) or weekly (`cron.weekly`) basis.

- `/etc/cups`: Contains files used to configure the CUPS printing service.

- `/etc/default`: Contains files that set default values for various utilities.
  For example, the file for the `useradd` command defines the default group
  number, home directory, password expiration date, shell, and skeleton
  directory (`/etc/skel`) used when creating new users.

- `/etc/httpd`: Contains a variety of files used to configure the behavior of your Apache
  web server (specifically, the httpd daemon process). (On Ubuntu and other Linux
  systems, /etc/apache or /etc/apache2 is used instead.)

- `/etc/mail`: Contains files used to configure your sendmail mail transport agent.
-
- `/etc/postfix`: Contains configuration files for the postfix mail transport agent.
-
- `/etc/ppp`: Contains several configuration files used to set up Point-to-Point Protocol
  (PPP) so that you can have your computer dial out to the Internet. (PPP was more
  commonly used when dial-up modems were popular.)

- `/etc/rc?.d`: There is a separate rc?.d directory for each valid system state: rc0.d
  (shutdown state), rc1.d (single-user state), rc2.d (multiuser state), rc3.d (mul-
  tiuser plus networking state), rc4.d (user-defined state), rc5.d (multiuser, net-
  working, plus GUI login state), and rc6.d (reboot state). These directories are
  maintained for compatibility with old UNIX SystemV init services.

**The following are some interesting configuration files in /etc:**

`aliases`: Can contain distribution lists used by the Linux mail services. (This file is
located in /etc/mail in Ubuntu when you install the sendmail package.)

`bashrc`: Sets system-wide defaults for bash shell users. (This may be called bash.

`bashrc` on some Linux distributions.)

crontab: Sets times for running automated tasks and variables associated with the
cron facility (such as the SHELL and PATH associated with cron).
csh.cshrc (or cshrc): Sets system-wide defaults for csh (C shell) users.
exports: Contains a list of local directories that are available to be shared by remote
computers using the Network File System (NFS).
fstab: Identifies the devices for common storage media (hard disk, DVD, CD-ROM,
and so on) and locations where they are mounted in the Linux system. This is used
by the mount command to choose which filesystems to mount when the system
first boots.
group: Identifies group names and group IDs (GIDs) that are defined on the system.
Group permissions in Linux are defined by the second of three sets of rwx (read,
write, execute) bits associated with each file and directory.
gshadow: Contains shadow passwords for groups.
host.conf: Used by older applications to set the locations in which domain names
(for example, redhat.com) are searched for on TCP/IP networks (such as the Inter-
net). By default, the local hosts file is searched and then any name server entries in
resolv.conf.
hostname: Contains the hostname for the local system (beginning in RHEL 7 and
recent Fedora and Ubuntu systems).
hosts: Contains IP addresses and hostnames that you can reach from your computer.
(Usually this file is used just to store names of computers on your LAN or small pri-
vate network.)
inittab: On earlier Linux systems, contained information that defined which pro-
grams start and stop when Linux boots, shuts down, or goes into different states in
between. This configuration file was the first one read when Linux started the init
process. This file is no longer used on Linux systems that support systemd.
mtab: Contains a list of filesystems that are currently mounted.
mtools.conf: Contains settings used by DOS tools in Linux.
named.conf: Contains DNS settings if you are running your own DNS server (bind or
bind9 package).
nsswitch.conf: Contains name service switch settings, for identifying where criti-
cal system information (user accounts, hostname-to-address mappings, and so on)
comes from (local host or via network services).
ntp.conf: Includes information needed to run the Network Time Protocol (NTP).
passwd: Stores account information for all valid users on the local system. Also
includes other information, such as the home directory and default shell. (Rarely
includes the user passwords themselves, which are typically stored in the /etc/
shadow file.)

[[adminLogFiles+systemdJournal.md]]

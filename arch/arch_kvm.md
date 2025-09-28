[ 11:05AM ] [ jr@archlinux:~/Downloads ]

```bash
$ sudo virsh -c qemu:///system net-define Whonix_external
```

```text
\*.xml error: Failed to define network from
Whonix_external_network.xml error: operation failed: network 'Whonix-External'
already exists with uuid ac15eb0e-0438-473e-a8b4-b985fc72cc58
```

[ 11:06AM ] [ jr@archlinux:~/Downloads ]

```bash
$ sudo virsh net-list --all
 Name              State    Autostart   Persistent
----------------------------------------------------
 default           active   yes         yes
 Whonix-External   active   yes         yes
 Whonix-Internal   active   yes         yes
```

## Name State Autostart Persistent

[ 11:07AM ] [ jr@archlinux:~/Downloads ]

```bash
$ sudo virsh net-destroy Whonix-External
```

Network Whonix-External destroyed

[ 11:07AM ] [ jr@archlinux:~/Downloads ]

```bash
$ sudo virsh net-undefine Whonix-External
```

Network Whonix-External has been undefined

[ 11:07AM ] [ jr@archlinux:~/Downloads ]

```bash
$ sudo virsh net-list --all
```

## Name State Autostart Persistent

default active yes yes Whonix-Internal active yes yes

[ 11:07AM ] [ jr@archlinux:~/Downloads ]

```bash
$ sudo virsh net-destroy Whonix-Internal
```

Network Whonix-Internal destroyed

[ 11:08AM ] [ jr@archlinux:~/Downloads ]

```bash
$ sudo virsh net-undefine Whonix-Internal
```

Network Whonix-Internal has been undefined

[ 11:08AM ] [ jr@archlinux:~/Downloads ]

```bash
$ sudo virsh net-list --all
```

## Name State Autostart Persistent

default active yes yes

# Change default shell on secureblue

```bash
run0 vim /etc/passwd
```

Edit the line with your user on it and change `:/bin/bash` to your desired
shell.

```passwd
root:x:0:0:root:/root:/bin/bash
jr:x:1000:1000:jr:/home/jr:/bin/fish
dnsconfd:x:962:962:Dnsconfd local DNS cache configurator:/:/sbin/nologin
```

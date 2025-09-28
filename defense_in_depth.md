# Defense in Depth

Defense in depth is the process of having multiple layers of security just in
case one fails. VMs are an example of defense in depth, a VM provides an extra
layer of security just in case the security of what is running inside the VM
fails. Multiple layers of security can slow down or even completely prevent an
adversary from compromising your system. When it comes to anonymity, layered
security is common, Tails for instance uses AppArmor to further restrict Tor
Browser just in case it is attacked. Whonix uses VMs to prevent de-anonymization
even in the event that a vulnerability in software such as Tor Browser is
exploited.

## Common Bad Practices

Defense in depth is not about 100% security, sometimes layers aren't needed. It
won't work if you are trying to defend against every single possible attack
under the sun. For example, increasing the length of Tor circuits to 6 relays,
there are people that want to do this, and it does not make you more anonymous.
An attacker is more likely to attack Tor Browser itself, then to do traffic
analysis on the Tor network. A general rule is if adding a layer of security is
going to have a minimal effect on increasing security, then you don't need it.

When it comes to encryption, many focus on the algorithm, in reality the keys
are what need to be protected the most. If a password for encryption is used,
the user should focus on using a strong password. As another layer of
protection, a strong KDF such as Argon2 should be used. That is much more
effective than adding more encryption algorithms. Be thoughtful about where you
apply your layers, don't add redundant layers that don't improve security.

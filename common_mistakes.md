# Common Mistakes

## 100% Security

A better approach is to have reasonable security and to keep up to date with the
latest threats through constant software updates and applying things such as
defense in depth. What this does is it makes the time and effort required not
worth it or impractical for your adversary. Cryptography does this, a 128-bit
key can be cracked, but it takes too long. Anonymity plays a role, it's harder
to attack someone who you don't know anything about.

Tails responded to someone claiming that their LUKS computer got cracked when
their VeraCrypt computer didn't. In reality, they were still using LUKS1 with a
password that couldn've been better obviously. They responded by announcing that
they switched to LUKS2 with Argon2 which are more expensive to password crack.
If his password was longer or if it was a passphrase of sufficient length, this
probably wouldn't have happened.

Rather than automatically believing or claiming that something is insecure, it
is better to investigate why it didn't protect something as well as it should
have, and then make changes accordingly. Sometimes things are insecure, and you
should stop using them, such as 1024 bit RSA, other times something wasn't used
or done properly, there are typically a lot of factors involved.

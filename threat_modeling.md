# Threat Modeling

Threat modeling is the process of getting to know your adversary, indentifying
information or attack vectors that your adversary might exploit or is capable of
exploiting, and then finding a way to mitigate some of those exploits your
adversary might use. When all of this info is compiled together, it is called a
threat model. Everyone has a different threat model and different ways of threat
modeling.

## How to develop a Threat Model

You need to know 3 things: who your adversary is, what they are capable of, and
what the adversary's goal is. If you know all 3, you're off to a great start.

Example: The FBI trying to deanonymize a Tor user.

Capabilities: We know they're capable of using NITs (drive-by-downloads) to
deanonymize Tor users.

What can you do?: Use Tails or Whonix rather than plain Tor Browser.

### STRIDE

STRIDE: Spoofing, Tampering, Repudiation, Information Disclosure, Denial of
Service, and Escalation of privileges.

Example question to ask yourself: Can my adversary spoof their identity? If yes,
then you may go through the process finding how your adversary would spoof an
identity and how to prevent or detect identity spoofing techniques.

### CIA Triad

CIA: Confidentiality, Integrity, and Availability. This threat modeling process
highlights the three main things you want to protect from an adversary.

- Confidentiality of info to prevent your adversary from knowing what it is.

- Integrity to prevent malicious modification of information or spoofing.

- Availability to prevent your adversary from making information unavailable to
  you. For instance, on a website with a login form, you want to keep the
  password database confidential. It also needs integrity, so someone
  unauthorized can't just change the password for a user, and it needs to stay
  available, so a user can always log in.

### Attack Trees

Attack trees are supposed to let you map out what attacks your adversary may
attempt to achieve, the adversary's goad, and how to mitigate them effectively.

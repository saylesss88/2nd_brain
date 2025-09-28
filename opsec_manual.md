From this point, you can study and learn about your adversary, including what
attacks and techniques are they going to try in order to get to what you are
trying to protect. From this point you can find mitigations and countermeasures
in order to slow down or stop your adversary entirely, keeping what you are
trying to protect safe.

In the context of anonymity, what you are trying to protect is most likely going
to be your true identity. However, this may vary from person to person depending
on the situation.

## File Based Encryption

Picocrypt CryFS

FBE is very hard to get around when done correctly. It often doesn't encrypt the
metadata enabling someone to possibly infer whats on the file. CryFS will hide
the most.

Can be used in areas where FDE can't like the cloud.

FBE also commonly includes authenticated encryption, which can detect malicous
modifications to encrypted data.

## Message Encryption

As long as the keys have not been swapped out during the key exchange (can be
verified by comparing fingerprints (OMEMO) or a safety number (Signal Protocol))
and neither of the endpoints are compromised. The messages are almost guaranteed
to not be read by anyone else but you and your recipient.

## Algorithms

Most modern ciphers are not currently broken and should be OK to use. This
includes the AES finalists such as Serpent, Twofish, RC6, MARS, and the AES
winner Rijndael.

The cipher of choice is AES (slightly modified version of Rijndael) because it
has had the most analysis and testing done on it and proper implementations have
practically never been broken and probably won't be for a very long time. AES
also uses hardware acceleration making it extremely fast on most processors,
causing little to no performance impact on the device it is used on.

ChaCha20 has now seen more use especially in TLS, it is also fast making it a
great alternative to AES in some systems. Serpent, Twofish, RC6, and MARS are
also good choices but shouldn't be your first. They aren't broken but have
received way less analysis and don't have any hardware accelerations, and so are
slow.

Ciphers to Avoid:

- RC4

- DES

- Tripple DES

- Blowfish

- IDEA

- Kuznyechik

## Tor / Tor Browser

WebRTC is practically not usable in Tor to prevent IP leaks and HTML canvas
elements are randomized to prevent fingerprinting.

Tor Browsers 3 security settings:

- The safer setting is more restrictive on CSS to prevent fingerprinting attacks
  and also disables WebAssembly and JavaScript JIT compiler, both of which are a
  large source of bugs and vulnerabilities.

- The safest setting disables almost everything and only keeps the necessary
  things for static web pages. At this setting JavaScript is disabled, and CSS
  is restricted in the same way as the Safer setting. The safest setting has the
  most minimal attack surface.

Tor and Tor Browser can keep you relatively anonymous but when it comes to
exploits and vulnerabilities, it may require some tweaking of the security
settings or more advanced tools such as Whonix or Tails. Also note that Tor
cannot protect all your communications, just because you route XMPP over Tor
doesn't mean that your messages are private, anonymous but not private.

### XMPP with OMEMO

XMPP is a messaging protocol that on the surface looks similar to email (it's
not similar at all). Different users on different servers can communicate with
each other across the internet. The only problem is that messages are not
private, server owners and anyone in between can easily view those messages.

This is where OMEMO comes in, OMEMO is an end-to-end encryption protocol
designed to be used with XMPP.

### Whonix

Whonix is a much safer way to use Tor anonymously. Whonix uses a two VM
approach, one VM for networking, one VM for browsing and other applications.
This way, in order for an adversary to deanonymize you, they not only have to
find a vulnerability in Tor Browser. They also have to find a vulnerability that
allows them to escape the VM which is extremely difficult to do.

Everything in Whonix is isolated from the rest of the machine, internet traffic
is forced through Tor with no way around since the networking is in a completely
separate VM. Whonix also comes with the Vanguards Tor plugin, designed to
prevent guard discovery and other traffic analysis attacks that may be used to
deanonymize you over a period of time.

#### Whonix Limitations

Using the same Whonix-Workstation VM for different purposes or anonymous
identities may allow an adversary to deanonymize you. Many users won't change
the sudo password of the Whonix-Workstation VM, while this doesn't allow a VM to
escape it makes it to attack. You should have multiple copies of
Whonix-Workstation for different purposes, you may also opt to use the live mode
for daily activities.

### Tails

Tails is a live system designed to not leave a trace of anything you do on the
PC the Tails USB was used on. The version of Tor Browser in Tails also comes
with an ad blocker. The Tor Browser in Tails has also gon through some
additional security hardening, mainly through the use of AppArmor. Tails forces
all traffic thorough Tor, traffic that refuses to go through TOr is simply
dropped.

Tails however makes it obvious that you are using Tails, the ad blocker in
Tails' Tor Browser is unique to the Tails OS. Tails works great against more
generic attacks that are used like a hand grenade (like NIT) but if you are
being actively targeted, Tails will have very limited use for you, Whonix would
be a better choice in such scenarios.

NOTE: It is still recommended by many to disable JavaScript while using Tails,
but if you are just browsing Reddit or doing normal generic stuff, disabling
JavaScript isn't needed and would be overkill in such scenerios.

## Mobile Device location tracking

Cellphones can be tracked without a SIM Card.

Cellphones will connect to the cell tower that provides the best signal, but
they will also still contact other towers in order to estimate their signal
strength. Whenever a cellphone contacts a tower, the tower takes note of the
time, device, and signal strength. By using this data from multiple towers, it
is trivial to pinpoint the location of a cellphone using simple maths. A
computer can automate this and perform it in less than a second, which may allow
real time location tracking.

5G has a lower range so it needs more towers closer together, which enables more
accurate tracking. Not having a 5G phone won't work either because most of those
towers are also 4G/LTE compatible, even if they weren't your phone would still
contact them to get info about the tower. It's not the type of signal, it's the
amount of towers and how close they are together that can determine how accurate
tracking can be.

You shouldn't trust your phones airplane-mode unless using Graphine OS. If it
has a removable battery, remove it, a phone with no power can't phone home.
Turning your phone off may work, but has proven to be unreliable with newer
smartphones, the problem is how do you know the phone is actually off.

Faraday bags, pouches, and cages provide the required isolation to prevent the
phone from communicating with anything outside, including cell towers. Foil is a
cheap alternative but depending on the phone it can take 5 all the way up to 20
layers to block all signal. Take note of how the bag closes if you get a faraday
bag, folding bags tend to wear out their inner signal blocking layer after a
month or two of use.

### Laptops

If you connect to free Wi-Fi, it's possible to track your movements by tracking
which Wi-Fi networks you connect to. Most free Wi-Fi will log the MAC address of
your computer, which usually stays persistent across networks. By using data
from Wi-Fi networks, it's possible to track a devices movement by knowing what
Wi-Fi networks it has connected to.

To prevent this form of tracking, the MAC address of the device should be
randomized. Also, if the Wi-Fi network has a captive portal, enter different
information upon every connection if the captive portal requires any
information.

Most Linux distributions allow some sort of MAC randomization through the use of
the network manager or through the macchanger package. If you opt for the
macchanger package, note that sometimes it doesn't work, verify the MAC has
changed before connecting to a Wi-Fi network.

Hardware Solutions

Some devices geared towards privacy come with hardware switches that can
disconnect things such as the microphone, camera, and wireless radios (wifi,
cell towers, and Bluetooth). These hardware switches are as effective as
physically disconnecting the microphone, camera, or wireless radio. These
switches are also much simpler to use and may be more reliable than a faraday
bag.

# Key Verification

All messaging apps encourage you to verify your keys out-of-band, i.e., off of
the messaging system itself.

So you should find multiple systems with which you can verify the keys. Without
definitively knowing who a key came from or is coming from, you can't rely on a
secure messaging system since it's not completely secure yet.

It's harder to fake someone's communications in more than one service.

When you ask for secret info within the channel itself ("in-band verification")
there's a risk that an imposter might secretly be talking to both parties,
called a man-in-the-middle or machine-in-the-middle attack. If you ask for a
fingerprint they can just send you theirs and you would never know.

You should verify keys when you use a new messaging tool to communicate, or when
someone's keys change. A person's keys might change when they get a new phone or
if they add a new device.

## Verifying Keys Out-Of-Band

To make verification easier, communication software can show you a fingerprint
or safety number based on the key.

To verify keys, your contact can read or show you the fingerprint of their key,
while you check it against the fingerprint of the key you have for them on your
device.

## Verifying in Person

This is ideal because it's easier to confirm someone is who they say they are
when you're face-to-face with them.

Many apps make a QR code available to make this easier. If not, it's worth it to
go through and verify every character of their fingerprint against what you have
for them.

## Verifying over another medium

If you can't meet in person, use a different method of communication to verify.

For example, if you're trying to verify Signal safety numbers with someone, you
could use the telephone or video chat, or even another end-to-end encrypted
communication app like WhatsApp. Using video chat makes impersonating you even
harder than using another text-based app.

You can either read your keys fingerprint aloud or you can copy-paste it into a
communications program. Whichever, it's important to check every single letter
and number.

Be on the lookout for when your friends keys change, be sure to confirm this
with them and re-verify their new keys.

## Symmetric Cryptography

When there is one key to both encrypt and decrypt, it is called symmetric
cryptography.

Protect a message with a "key of three", shifting the letters down the alphabet
by three. For example, A would be D, B would be E, and so on.

The method of shifting the alphabet by three characters is a historic example of
encryption used by Julius Caesar, hence, the _Caesar cipher_.

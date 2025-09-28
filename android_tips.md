## Other Android Security Tips

{{< details title=" ✔️ Click to Expand Android Security Tips Section">}}

If you don't like the idea of someone forcing you to unlock your phone so they
can sift through your data:

- It's often recommended to **not** use biometrics as they can be forcibly be
  taken from you while a password typically can't.

- Cellibrite relies on your phone being in AFU mode which is After first unlock.

- Set the Auto optimization (Auto Restart) to the shortest amount of time
  possible. When your phone first reboots it is in BFU mode which is Before
  first unlock mode. Biometrics are usually disabled in this mode and the
  encryption keys are **not** saved in RAM making the attack surface much
  smaller.(i.e., they typically have to guess the password or brute-force it)
  - That said, if someone gets your phone while it's in AFU mode they will only
    have the amount of time until your next auto reboot to try to extract your
    data.

- You can also try to reduce the number of unlock attempts before the device
  wipes. The default is typically 20 attempts, I haven't found a way to lower
  this.

- You can't really trust Airplane mode on anything but GrapheneOS. If you are
  able to remove your battery, do that when it matters.

- Use a password manager instead of the in browser manager.

{{</details>}}

### Troubleshooting

I found that it's a bit more straightforward strictly using Orbot for this. It
could be a skill issue on my part but opening Orbot, `Choose apps`, search
`DuckDuckGo`. Select DDG.

Open DDG and search:

```text
https://dnsleaktest.com
```

Brandenburg, Germany

### Manual Configuration

{{< details title=" ✔️ Click to Expand Manual Proxy Section">}}

**SOCKS5 Proxy Example**:

On the first opening of Orbot, don't start it. First go to `More`,
`Orbot Settings`, and check `Power User Mode`. This enables Orbot to start
without VPN mode preventing conflicts, you'll notice the Orbot Home Screen shows
`Connect` rather than `Connect to VPN`.

Open `Orbot -> More`: On the bottom of the screen you'll see the Proxy Ports.
`HTTP: 8118` - `SOCKS: 9050`, these are the Port numbers. We will check them
against Rethinks default settings, which are correct as they are.

Go to `Configure -> Proxy` and enable `OTHER -> Setup SOCKS5 Proxy`:

It says "The selected app will be excluded from the VPN to let it proxy
connections on your behalf."

Based on that, the only App that it makes sense to select from the dropdown is
`Orbot`. The default Port Number `9050` matches Orbots `SOCKS: 9050`.

Ensure that Orbot shows `Connected` and go to Rethinks Home Screen. It should
show, `Protected with SOCKS PROXY`.

I have had the most success when I follow these steps:

- Have the `OTHER -> Setup SOCKS5 Proxy` configured and ready to go but off
  right now.

- Start Orbot in `Connected` mode through the `Power User Mode`.

- If Rethink isn't running, start Rethink without the SOCKS proxy enabled. When
  below the STOP button shows `Protected`, go to `OTHER -> Setup SOCKS5` and
  enable it.

- Go back to Rethinks HOME and below the STOP button should show
  `PROTECTED WITH SOCKS PROXY`

- Beyond this I have had connectivity issues and have failed to reach
  `dnsleaktest.com` with DDG so far.

## {{</details>}}

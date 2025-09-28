# Rethink DNS

RethinkDNS is a DNS Resolver service with custom rules and blocklists.

A **DNS resolver** is an address book of the internet, it helps locate IP
addresses of the servers given a domain name. For example, dns.google.com (a
domain name) is located at 8.8.8.8 (IP address). This mapping is retrieved by a
DNS resolver.

You can configure Rethink in your device / internet browser that supports Secure
DNS (aka DNS over HTTPS).

## Configure a Custom DNS resolver with Custom blocklists through Rethink website

These are **not** for devices that have the app on them. You can use these on
your computer for example.

1. Go to: [RethinkDNS Configure](https://www.rethinkdns.com/configure)

2. Use either the `simple ->` for groups of blocklists, or `advanced ->` for
   more fine grained control.

3. Once you have them all selected, decide if you want to use DoH or DoT by
   clicking the `DoH` button under the Rethink Logo.

- DoH resolver addresses' look like: `https://sky.rethinkdns.com/`

- DoT resolver addresses' look like: `1-cbycee6juakjaaa`

For Firefox, open Settings, Privacy & Security, scroll down to Enable DNS over
HTTPS using: Max Protection, Custom, and enter `https://sky.rethinkdns.com/`

Firefox doesn't support DoT natively yet.

## Rethink on Android

**RethinkDNS takes over**:

1. VPN Slot: RethinkDNS works by creating a local VPN on your device. It's not a
   traditional VPN that routes your traffic to a remote server. Instead, it
   creates a secure tunnel on your phone that all network traffic (including DNS
   queries) must pass through.

2. Centralized DNS Handling: Because it occupies the local VPN slot, it
   intercepts all DNS requests from every app on your device.

3. App-Level Control: Unlike Android's Private DNS, which is a system-wide
   setting, Rethink gives you more granular control over how each individual app
   handles its network traffic. This enables you to:

- Force all apps to use the same DNS server you've configured through Rethink.

- Block apps that try to bypass your settings.

- Apply different rules to different apps.

- Analyze and log the DNS and network activity for every app, giving you a clear
  view of what your phone is doing in the background.
  - I have never used the Microsoft Link to Windows and even went into settings
    and disabled it and force stopped it and Link to Windows is still the most
    blocked app on my device constantly trying to phone home.

## Rethink Rules

- Go to `Configure -> Apps`, and tap the 🛜📶 to block all apps.

- Now, search for the apps you use and either Bypass Universal, or Isolate them.

- Bypass Universal the `Google Play services` app.

## Rethink Firewall

With the Firewall, you can set Universal Rules.

Go to Configure, Firewall, Universal firewall rules and set:

- Block all apps when device is locked

- Block newly Installed Apps

- I block any app not in use and block port 80 (insecure HTTP) traffic

- From here you can get more restrictive if you so choose

- Many of these tips come from the following Forum:
  - [GrapheneOS Discussion Forum on Rethink](https://discuss.grapheneos.org/d/12728-proton-apps-pinging-google-api-sending-reports-back-after-opting-out/54)

## Understanding DNS over HTTPS and DNS over TLS in the RethinkDNS App

It's my understanding that the website is for computer use and you use the
RethinkDNS app for your phone. They are completely separate and not used
together.

When you configure and enable Rethink to control DNS over HTTPS, if your browser
is also enforcing strict DNS over HTTPS to a different DNS resolver, they will
be blocked by Rethink as a `DNS bypass`.

For Android Firefox, switch the DNS over HTTPS setting to "Default Protection
Firefox will use your system's DNS resolver". This will allow Firefox to use
Rethink's DNS resolver.

## Using a Custom DNS in the App

Go to Configure, DNS, Other DNS. From there you have quite a few choices.

Let's say you chose DoT for DNS-over-TLS, from there you can choose between 5
providers. Mullvad has a good reputation for keeping minimal data.

If Firefox is at the default DNS over HTTPS setting, it should now use DoT
through the RethinkDNS app.

## Logs

Go to Configure, Logs, and try to access the app that's not working. You should
see said app at the top of the Network Logs, click it. In the top right of the
tab, you'll see the reason why it's not working such as: `App Blocked`, or
`DNS Bypass`.

Once you click on the log of the app in question, you'll be given 3 drop down
options. If you set an app to Bypass DNS and Firewall settings, you will see
that in the first dropdown box.

The next drop down is 'Block,trust this IP for this app' where you can set a
rule to 'Block' or 'Trust'.

Apps like Reddit rely on many third-party services, backend APIs etc. to work.
It's my understanding that this fine grained control isn't fully worked out yet
and some connections or domains will stay blocked even with an explicit Trust
Rule. In the long run I had to exclude Reddit for it to work but for most apps,
allowing it to bypass DNS and Firewall rules is sufficient.
